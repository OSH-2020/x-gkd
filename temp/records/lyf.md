# I 纠删码相关说明文档

### 一、相关文档链接

Github项目：https://github.com/darrenldl/reed-solomon-erasure

Rust docs：[Crate reed_solomon_erasure](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/)

### 二、使用

#### Rust usage（摘自Github）

Add the following to your `Cargo.toml` for the normal version (pure Rust version)

```
[dependencies]
reed-solomon-erasure = "4.0"
```

or the following for the version which tries to utilise SIMD

```
[dependencies]
reed-solomon-erasure = "4.0"
```

and the following to your crate root

```
extern crate reed_solomon_erasure;
```
示例程序：

```rust
#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;
// or use the following for Galois 2^16 backend
// use reed_solomon_erasure::galois_16::ReedSolomon;

fn main () {
    let r = ReedSolomon::new(3, 2).unwrap(); // 3 data shards, 2 parity shards

    let mut master_copy = shards!(
        [0, 1,  2,  3],
        [4, 5,  6,  7],
        [8, 9, 10, 11],
        [0, 0,  0,  0], // last 2 rows are parity hards
        [0, 0,  0,  0]
    );

    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    shards[0] = None;
    shards[4] = None;

    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    assert!(r.verify(&result).unwrap());
    assert_eq!(master_copy, result);
}
```

#### 可能出现的问题

windows 环境中，下载`reed-solomon-erasure` 可能出现的问题：

![](C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 133749.png)

解决方法：

将以下内容添加到 `Cargo.toml` 中

```
[http]
check-config = false
```

### 三、struct reed solomon erasure::ReedSolomon的说明

链接：Rust docs ：Struct [reed_solomon_erasure](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/index.html)::[ReedSolomon](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/struct.ReedSolomon.html)

```
pub struct ReedSolomon<F: Field> { /* fields omitted */ }
```

该结构体是一个 Reed-Solomon erasure code encoder/decoder.

注：

- data shards 数据碎片 parity shards 奇偶校验碎片

- 方法结尾处有 `_sep` 表示对数据碎片采用不变的引用，对奇偶校验碎片采用可变的引用。（对数据碎片只读）

- 方法结尾处有`_single` 表示 shard by shard encoding，即奇偶校验碎片是一次使用一个数据碎片部分构建的，**容易错误使用**。

| methods            | 说明 |
| ------------------ | ---- |
|**new**				| pub fn new(data_shards: usize,  parity_shards: usize) -> Result<ReedSolomon\<F>,Error> |
| data_shard_count   | 返回data shard的数目 |
| parity_shard_count | 返回parity shard的数目 |
| total_shard_count  | 返回total shard的数目 |
| **encode**         | Constructs the parity shards. （The slots where the parity shards sit at will be overwritten.） |
| encode_sep         | Constructs the parity shards using a read-only view into the data shards. （The slots where the parity shards sit at will be overwritten.） |
| encode_single      | 比encode方法多一个参数i_data索引，只用该索引指向的数据碎片构建奇偶校验碎片 |
| encode_single_sep  |      |
| **reconstruct**    | Reconstructs all shards |
| reconstruct_data   | Reconstructs only the data shards. |
| **verify**         | 核验奇偶校验碎片是否正确 |
| verify_with_buffer | 每次verify时，需开辟一段内存空间，存储根据数据碎片计算出的奇偶校验碎片内容，再与原内容比较，以判断是否正确。verify_with_buffer 是将buffer指定为这段内存空间，避免重复开辟。 |

注：加粗的为常用方法

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151419.png" style="zoom: 50%;" />

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151349.png" style="zoom: 50%;" />



<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151435.png" style="zoom:50%;" />

![](C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151455.png)

### 四、需要改写的部分

上述方法不是针对文件的，原17级项目也改写了backblaze的 encoder和decoder模块，我们也需要改写成适合文件的。

原 17 级设计报告中涉及纠删码模块的部分如下：

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 141609.png" style="zoom: 60%;" />

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 141624.png" style="zoom:60%;" />

参考 17 级源代码中的encoder，他们的做法是：FILE -> FileInputStream -> InputStream -> shards \[][] -> 调用 -> FileOutputStream -> OutputStream -> write 到指定文件中

改写难点：

- Rust 文件输入流处理

- backblaze 的纠删码算法中的方法定义可能与 Rust中的纠删码算法不太一样。

# II SQL 笔记

廖雪峰 SQL 教程原网址：https://www.liaoxuefeng.com/wiki/1177760294764384

菜鸟教程：https://www.runoob.com/mysql/mysql-tutorial.html

此笔记摘录自廖雪峰的 SQL 基础教程

## 1 关系数据库描述

**数据模型：**关系模型——二维表格

**数据类型：**（每一列的数据类型）

![](C:\Users\12935\Pictures\Screenshots\批注 2020-05-18 214309.png)

别名：`REAL` = `FLOAT(24)`。还有一些不常用的数据类型，例如，`TINYINT`（范围在0~255）。各数据库厂商还会支持特定的数据类型，例如`JSON`。

**主流关系数据库：**

1. 商用数据库，例如：[Oracle](https://www.oracle.com/)，[SQL Server](https://www.microsoft.com/sql-server/)，[DB2](https://www.ibm.com/db2/)等；
2. 开源数据库，例如：[MySQL](https://www.mysql.com/)，[PostgreSQL](https://www.postgresql.org/)等；
3. 桌面数据库，以微软[Access](https://products.office.com/access)为代表，适合桌面应用程序使用；
4. 嵌入式数据库，以[Sqlite](https://sqlite.org/)为代表，适合手机应用和桌面程序。

**SQL定义：**

SQL：结构化查询语言的缩写。虽然SQL已经被ANSI组织定义为标准，大部分数据库都在标准的SQL上做了扩展，扩展为“方言”。

**语法特点：**SQL语言关键字不区分大小写！！！但是，针对不同的数据库，对于表名和列名，有的数据库区分大小写，有的数据库不区分大小写。同一个数据库，有的在Linux上区分大小写，有的在Windows上不区分大小写。

**约定：SQL关键字总是大写，以示突出，表名和列名均使用小写。**

## 2 安装 MySQL

MySQL内部还包含了多种数据引擎，常用的包括：

- InnoDB：由Innobase Oy公司开发的一款支持事务的数据库引擎，2006年被Oracle收购；
- MyISAM：MySQL早期集成的默认数据库引擎，不支持事务。

使用MySQL时，不同的表还可以使用不同的数据库引擎。不知如何选择引擎时，记住总是选择*InnoDB*即可

MySQL 官方版本有 4 个

下载链接：https://dev.mysql.com/downloads/mysql/ （community版本）

win10下MySQL zip安装常见问题：https://blog.csdn.net/sinat_34461975/article/details/86664760

```
//命令提示符中cd到bin目录，例如：
E:\Program Files (x86)\mysql-8.0.20-winx64\bin>mysql -u root -p
Enter password: ********
```

## 3 关系模型

行为**记录（Record）**，列为**字段（Colomn）**。

字段定义了数据类型（整型、浮点型、字符串、日期等），以及是否允许为`NULL`。**注意`NULL`表示字段数据不存在。一个整型字段如果为`NULL`不表示它的值为`0`，同样的，一个字符串型字段为`NULL`也不表示它的值为空串`''`。**

**通常情况下，字段应该避免允许为NULL。**不允许为NULL可以简化查询条件，加快查询速度，也利于应用程序读取数据后无需判断是否为NULL。

### 3.1 主键 id

主键：指能够通过某个字段唯一区分出关系表中的不同记录，这个字段被称为*主键*。

- 记录一旦插入到表中，主键最好不要再修改。

- 选取主键的一个基本原则是：不使用任何业务相关的字段作为主键。

作为主键的字段一般命名为`id`。常见的可作为`id`字段的类型有：

1. 自增整数类型：数据库会在插入数据时自动为每一条记录分配一个自增整数，这样我们就完全不用担心主键重复，也不用自己预先生成主键；
2. 全局唯一GUID类型：使用一种全局唯一的字符串作为主键，类似`8f55d96b-8acc-4636-8cb8-76bf8abc2f57`。GUID算法通过网卡MAC地址、时间戳和随机数保证任意计算机在任意时间生成的字符串都是不同的，大部分编程语言都内置了GUID算法，可以自己预算出主键。

eg.可在`students`表中定义主键为`BIGINT NOT NULL AUTO_INCREMENT`类型。

*注意INT自增类型的上限。*

联合主键：通过多个字段唯一标识记录，即两个或更多的字段都设置为主键，这种主键被称为联合主键。（尽量不使用联合主键）

```SQL
CREATE TABLE statistics (
    id BIGINT NOT NULL AUTO_INCREMENT,
    class_id BIGINT NOT NULL,
    average DOUBLE NOT NULL,
    PRIMARY KEY (id)
);
```

**小结：**

主键是关系表中记录的唯一标识。主键的选取非常重要：**主键不要带有业务含义，而应该使用BIGINT自增或者GUID类型。主键也不应该允许`NULL`。**

可以使用多个列作为联合主键，但联合主键并不常用。

### 3.2 外键

**定义外键约束**：

```
ALTER TABLE students
ADD CONSTRAINT fk_class_id		
FOREIGN KEY (class_id)
REFERENCES classes (id);
```

其中，外键约束的名称`fk_class_id`可以任意，`FOREIGN KEY (class_id)`指定了`class_id`作为外键，`REFERENCES classes (id)`指定了这个外键将关联到`classes`表的`id`列（即`classes`表的主键）。

作用：通过定义外键约束，关系数据库可以保证无法插入无效的数据。即如果`classes`表不存在`id=99`的记录，`students`表就无法插入`class_id=99`的记录。

注：由于外键约束会降低数据库的性能，大部分互联网应用程序为了追求速度，并不设置外键约束，而是仅靠应用程序自身来保证逻辑的正确性。这种情况下，`class_id`仅仅是一个普通的列，只是它起到了外键的作用而已。

**删除一个外键约束**：

```
ALTER TABLE students
DROP FOREIGN KEY fk_class_id;
```

注意：删除外键约束并没有删除外键这一列。删除列是通过`DROP COLUMN ...`实现的。

**多对多：**

多对多关系实际上是通过两个一对多关系实现的，即通过一个中间表，关联两个一对多关系，就形成了多对多关系

**一对一：**可以将一个大表拆成两个表 -> 提高查询速度

### 3.3 索引

索引是关系数据库中对某一列或多个列的值进行预排序的数据结构。通过使用索引，可以让数据库系统不必扫描整个表，而是直接定位到符合条件的记录，这样就大大加快了查询速度。

如果要经常根据`score`列进行查询，就可以对`score`列创建索引：

```
ALTER TABLE students
ADD INDEX idx_score (score);
```

使用`ADD INDEX idx_score (score)`就创建了一个名称为`idx_score`，使用列`score`的索引。索引名称是任意的，索引如果有多列，可以在括号里依次写上，例如：

```
ALTER TABLE students
ADD INDEX idx_name_score (name, score);
```

索引的效率取决于索引列的值是否散列，即该列的值如果越互不相同，那么索引效率越高。

**唯一索引：**UNIQUE INDEX

例如，我们假设`students`表的`name`不能重复：

```
ALTER TABLE students
ADD UNIQUE INDEX uni_name (name);
```

通过`UNIQUE`关键字我们就添加了一个唯一索引。

也可以只对某一列添加一个唯一约束而不创建唯一索引：

```
ALTER TABLE students
ADD CONSTRAINT uni_name UNIQUE (name);
```

这种情况下，`name`列没有索引，但仍然具有唯一性保证。

## 4 查询数据 SELECT

### 4.1 基本查询

```SQL
SELECT * FROM <表名>		//查询一个表的所有行和所有列的数据
SELECT * FROM students;	 //例子：查询出`students`表的所有数据
```

`*`表示“所有列”，`FROM`表示将要从哪个表查询。

注意：**查询结果也是一个二维表，它包含列名和每一行的数据。**

`SELECT`语可用作计算，并不要求一定要有`FROM`子句。

```
SELECT 100+200;
```

```
//结果
100 + 200
300
```

用处：测试数据库连接是否有效。（许多检测工具会执行一条`SELECT 1;`来测试数据库连接）

### 4.2 条件查询

```
SELECT * FROM <表名> WHERE <条件表达式>		//语法

SELECT * FROM students WHERE score >= 80;	//例子
SELECT * FROM students WHERE score >= 80 AND gender = 'M';
SELECT * FROM students WHERE score >= 80 OR gender = 'M';
SELECT * FROM students WHERE NOT class_id = 2;
SELECT * FROM students WHERE (score < 80 OR score > 90) AND gender = 'M';
```

优先级：括号 > NOT > AND> OR 

**常用的条件表达式**

| 条件                 | 表达式举例1     | 表达式举例2      | 说明                                              |
| :------------------- | :-------------- | :--------------- | :------------------------------------------------ |
| 使用=判断相等        | score = 80      | name = 'abc'     | 字符串需要用单引号括起来                          |
| 使用>判断大于        | score > 80      | name > 'abc'     | 字符串比较根据ASCII码，中文字符比较根据数据库设置 |
| 使用>=判断大于或相等 | score >= 80     | name >= 'abc'    |                                                   |
| 使用<判断小于        | score < 80      | name <= 'abc'    |                                                   |
| 使用<=判断小于或相等 | score <= 80     | name <= 'abc'    |                                                   |
| 使用<>判断不相等     | score <> 80     | name <> 'abc'    |                                                   |
| 使用LIKE判断相似     | name LIKE 'ab%' | name LIKE '%bc%' | %表示任意字符，例如'ab%'将匹配'ab'，'abc'，'abcd' |

 查询分数在60分(含)～90分(含)之间的学生可以使用的WHERE语句是：

 WHERE score >= 60 AND score <= 90

WHERE score BETWEEN 60 AND 90

### 4.3 投影查询

投影查询：返回指定列

```
SELECT 列1, 列2, 列3 FROM ...		//结果只包含指定列
SELECT id, score, name FROM students;//例子

SELECT 列1 别名1, 列2 别名2, 列3 别名3 FROM ...//给列起别名
SELECT id, score points, name FROM students;//score别名points
SELECT id, score points, name FROM students WHERE gender = 'M';
```

起别名用来为结果集的列重命名

### 4.4 排序

查询结果通常按照主键排序

```
//ORDER BY 
SELECT id, name, gender, score FROM students ORDER BY score;

//DESC表示“倒序”
SELECT id, name, gender, score FROM students ORDER BY score DESC;
```

如果`score`列有相同的数据，要进一步排序，可以继续添加列名。例如，使用`ORDER BY score DESC, gender`表示先按`score`列倒序，如果有相同分数的，再按`gender`列排序：

```
SELECT id, name, gender, score FROM students ORDER BY score DESC, gender;
```

默认的排序规则是`ASC`：“升序”，即从小到大。`ASC`可以省略

如果有`WHERE`子句，那么`ORDER BY`子句要放到`WHERE`子句后面。

```
SELECT id, name, gender, score
FROM students
WHERE class_id = 1
ORDER BY score DESC;
```

### 4.5 分页

分页显示

分页实际上就是从结果集中“截取”出第M~N条记录。这个查询可以通过`LIMIT <M> OFFSET <N>`子句实现

```
//查询第一页：OFFSET 0
SELECT id, name, gender, score
FROM students
ORDER BY score DESC
LIMIT 3 OFFSET 0;

//查询第二页：OFFSET 3
SELECT id, name, gender, score
FROM students
ORDER BY score DESC
LIMIT 3 OFFSET 3;

//查询第三页：OFFSET 6
SELECT id, name, gender, score
FROM students
ORDER BY score DESC
LIMIT 3 OFFSET 6;
```

每页需要显示的结果数量`pageSize`（这里是3），

根据当前页的索引`pageIndex`（从1开始），确定`LIMIT`和`OFFSET`应该设定的值：

- `LIMIT`总是设定为`pageSize`；
- `OFFSET`计算公式为`pageSize * (pageIndex - 1)`。

若 OFFSET 超过查询的最大数量，不会报错，会得到一个空结果集

```
Empty result set	//空结果集
```

注意：

`OFFSET`是可选的，如果只写`LIMIT 15`，那么相当于`LIMIT 15 OFFSET 0`。

在MySQL中，`LIMIT 15 OFFSET 30`还可以简写成`LIMIT 30, 15`。

使用`LIMIT <M> OFFSET <N>`分页时，随着`N`越来越大，查询效率也会越来越低。

### 4.6 聚合查询

聚合查询：使用聚合函数（eg.统计总数、平均数）进行查询。

例：查询`students`表一共有多少条记录，我们可以使用SQL内置的`COUNT()`函数查询：

```
SELECT COUNT(*) FROM students;
```

结果：注意是个二维表

```
COUNT(*)
10
```

```
SELECT COUNT(*) num FROM students;	//使用聚合查询时，给列名起别名
```

`COUNT(*)`和`COUNT(id)`实际上是一样的效果（主键）

```
//与 where 结合
SELECT COUNT(*) boys FROM students WHERE gender = 'M';
```

| 函数 | 说明                                   |
| :--- | :------------------------------------- |
| SUM  | 计算某一列的合计值，该列必须为数值类型 |
| AVG  | 计算某一列的平均值，该列必须为数值类型 |
| MAX  | 计算某一列的最大值                     |
| MIN  | 计算某一列的最小值                     |

注意，`MAX()`和`MIN()`函数并不限于数值类型。如果是字符类型，`MAX()`和`MIN()`会返回排序最后和排序最前的字符。

```
SELECT AVG(score) average FROM students WHERE gender = 'M';
```

如果聚合查询的`WHERE`条件没有匹配到任何行，`COUNT()`会返回0，而`SUM()`、`AVG()`、`MAX()`和`MIN()`会返回`NULL`：

- 每页3条记录，如何通过聚合查询获得总页数？

```
SELECT CEILING(COUNT(*) / 3) FROM students;
```

分组聚合

```
SELECT COUNT(*) num FROM students GROUP BY class_id;
```

结果：

| num  |
| :--- |
| 4    |
| 3    |
| 3    |

执行该`SELECT`语句时，会把`class_id`相同的列先分组，再分别计算，因此，得到了3行结果

```
SELECT class_id, COUNT(*) num FROM students GROUP BY class_id;
```

| class_id | num  |
| :------- | :--- |
| 1        | 4    |
| 2        | 3    |
| 3        | 3    |

**聚合查询的列中，只能放入分组的列。**

使用多个列分组：

```
SELECT class_id, gender, COUNT(*) num FROM students GROUP BY class_id, gender;
```

### 4.7 多表查询

```
SELECT * FROM <表1> <表2>
```

又称“笛卡尔查询”，查询的结果也是一个二维表，是`students`表和`classes`表的“乘积”，即`students`表的每一行与`classes`表的每一行都两两拼在一起返回。结果集的列数是`students`表和`classes`表的列数之和，行数是`students`表和`classes`表的行数之积。



给列起别名：

```
SELECT
    students.id sid,
    students.name,
    students.gender,
    students.score,
    classes.id cid,
    classes.name cname
FROM students, classes;
```

格式：`表名.列名 别名`

也可先给表起别名，再用表的别名，给列起别名（更简洁）：

`FROM`子句给表设置别名的语法是`FROM <表名1> <别名1>, <表名2> <别名2>`。

```
SELECT
    s.id sid,
    s.name,
    s.gender,
    s.score,
    c.id cid,
    c.name cname
FROM students s, classes c;
```

```
//使用where条件
SELECT
    s.id sid,
    s.name,
    s.gender,
    s.score,
    c.id cid,
    c.name cname
FROM students s, classes c
WHERE s.gender = 'M' AND c.id = 1;
```

### 4.8 连接查询

连接查询对多个表进行JOIN运算，简单地说，就是先确定一个主表作为结果集，然后，把其他表的行有选择性地“连接”在主表结果集上。

- INNER JOIN 内连接

```
SELECT s.id, s.name, s.class_id, c.name class_name, s.gender, s.score
FROM students s
INNER JOIN classes c
ON s.class_id = c.id;
```

注意INNER JOIN查询的写法是：

1. 先确定主表，仍然使用`FROM <表1>`的语法；
2. 再确定需要连接的表，使用`INNER JOIN <表2>`的语法；
3. 然后确定连接条件，使用`ON <条件...>`，这里的条件是`s.class_id = c.id`，表示`students`表的`class_id`列与`classes`表的`id`列相同的行需要连接；
4. 可选：加上`WHERE`子句、`ORDER BY`等子句。

- OUTER JOIN 外连接

```
SELECT s.id, s.name, s.class_id, c.name class_name, s.gender, s.score
FROM students s
RIGHT OUTER JOIN classes c
ON s.class_id = c.id;
```

INNER JOIN只返回同时存在于两张表的行数据，由于`students`表的`class_id`包含1，2，3，`classes`表的`id`包含1，2，3，4，所以，INNER JOIN根据条件`s.class_id = c.id`返回的结果集仅包含1，2，3。

RIGHT OUTER JOIN返回右表都存在的行。如果某一行仅在右表存在，那么结果集就会以`NULL`填充剩下的字段。

LEFT OUTER JOIN则返回左表都存在的行。如果我们给students表增加一行，并添加class_id=5，由于classes表并不存在id=5的行，所以，LEFT OUTER JOIN的结果会增加一行，对应的`class_name`是`NULL`：

FULL OUTER JOIN，它会把两张表的所有记录全部选择出来，并且，自动把对方不存在的列填充为NULL：

**小结：**

JOIN查询需要先确定主表，然后把另一个表的数据“附加”到结果集上；

INNER JOIN是最常用的一种JOIN查询，它的语法是`SELECT ... FROM <表1> INNER JOIN <表2> ON <条件...>`；

JOIN查询仍然可以使用`WHERE`条件和`ORDER BY`排序。

## 5 修改数据

增删改查：CRUD：Create、Retrieve、Update、Delete

- INSERT：插入新记录；
- UPDATE：更新已有记录；
- DELETE：删除已有记录。

### 5.1 INSERT

`INSERT`：一次向一个表中插入一条或多条记录

基本语法：

```SQL
INSERT INTO <表名> (字段1, 字段2, ...) VALUES (值1, 值2, ...);

INSERT INTO students (class_id, name, gender, score) VALUES (2, '大牛', 'M', 80);
//一次性添加多条记录
INSERT INTO students (class_id, name, gender, score) VALUES
  (1, '大宝', 'M', 87),
  (2, '二宝', 'M', 81);
```

注意到我们并没有列出`id`字段，也没有列出`id`字段对应的值，这是因为`id`字段是一个自增主键，它的值可以由数据库自己推算出来。此外，如果一个字段有默认值，那么在`INSERT`语句中也可以不出现。

### 5.2 UPDATE

UPDATE：一次更新表中的一条或多条记录。

```sql
UPDATE <表名> SET 字段1=值1, 字段2=值2, ... WHERE ...;

UPDATE students SET name='大牛', score=66 WHERE id=1;
//一次更新多条记录
UPDATE students SET name='小牛', score=77 WHERE id>=5 AND id<=7;
//更新字段时使用表达式
UPDATE students SET score=score+10 WHERE score<80;
```

如果`WHERE`条件没有匹配到任何记录，`UPDATE`语句不会报错，也不会有任何记录被更新。

特别小心：`UPDATE`语句可以没有`WHERE`条件。这时，整个表的所有记录都会被更新。所以，在执行`UPDATE`语句时，最好先用`SELECT`语句来测试`WHERE`条件是否筛选出了期望的记录集，然后再用`UPDATE`更新。

注：在使用MySQL这类真正的关系数据库时，`UPDATE`语句会返回更新的行数以及`WHERE`条件匹配的行数。

### 5.3 DELETE

DELETE：一次删除表中的一条或多条记录。

```sql
DELETE FROM <表名> WHERE ...;

DELETE FROM students WHERE id=1;
//一次删除多条记录
DELETE FROM students WHERE id>=5 AND id<=7;
```

`DELETE`语句也可以一次删除多条记录

如果`WHERE`条件没有匹配到任何记录，`DELETE`语句不会报错，也不会有任何记录被删除。

特别小心：不带`WHERE`条件的`DELETE`语句会删除整个表的数据。在执行`DELETE`语句时也要非常小心，最好先用`SELECT`语句来测试`WHERE`条件是否筛选出了期望的记录集，然后再用`DELETE`删除。

注：在使用MySQL这类真正的关系数据库时，`DELETE`语句也会返回删除的行数以及`WHERE`条件匹配的行数。

## 6 MySQL

打开命令提示符，cd到mysql的bin目录下，输入命令`mysql -u root -p`，输入password。若正确，则连上了MySQL Server，同时提示符变为`mysql>`：

断开与MySQL Server的连接：`exit`

 MySQL Client的可执行程序是mysql，MySQL Server的可执行程序是mysqld。

```ascii
┌──────────────┐  SQL   ┌──────────────┐
│ MySQL Client │───────>│ MySQL Server │
└──────────────┘  TCP   └──────────────┘
```

在MySQL Client中输入的SQL语句通过TCP连接发送到MySQL Server。默认端口号是3306，即如果发送到本机MySQL Server，地址就是`127.0.0.1:3306`。

连接远程 MySQL Server 的命令：

```
//假设远程MySQL Server的IP地址是`10.0.1.99`，那么就使用`-h`指定IP或域名：
mysql -h 10.0.1.99 -u root -p
```

命令行程序`mysql`实际上是MySQL客户端，真正的MySQL服务器程序是`mysqld`，在后台运行。

### 6.1 管理MySQL

#### **数据库**

在一个运行MySQL的服务器上，可以创建多个数据库（Database）。

- 列出所有数据库 SHOW DATABASES

```
mysql> SHOW DATABASES;
+--------------------+
| Database           |
+--------------------+
| information_schema |
| mysql              |
| performance_schema |
| shici              |
| sys                |
| test               |
| school             |
+--------------------+
```

其中，`information_schema`、`mysql`、`performance_schema`和`sys`是系统库，不要改动。其他的是用户创建的数据库。

- 创建一个新数据库 CREATE DATABASE <数据库名>

```
mysql> CREATE DATABASE test;
Query OK, 1 row affected (0.01 sec)
```

- 删除一个数据库 DROP DATABASE<数据库名>

```
mysql> DROP DATABASE test;
Query OK, 0 rows affected (0.01 sec)
```

注意：删除一个数据库将导致该数据库的所有表全部被删除。

- 切换为当前数据库 USE <表名>

```
mysql> USE test;
Database changed
```

#### **表**

- 列出当前数据库的所有表 SHOW TABLES

```
mysql> SHOW TABLES;
+---------------------+
| Tables_in_test      |
+---------------------+
| classes             |
| statistics          |
| students            |
| students_of_class1  |
+---------------------+
```

- 查看一个表的结构 DESC <表名>

```
mysql> DESC students;
+----------+--------------+------+-----+---------+----------------+
| Field    | Type         | Null | Key | Default | Extra          |
+----------+--------------+------+-----+---------+----------------+
| id       | bigint(20)   | NO   | PRI | NULL    | auto_increment |
| class_id | bigint(20)   | NO   |     | NULL    |                |
| name     | varchar(100) | NO   |     | NULL    |                |
| gender   | varchar(1)   | NO   |     | NULL    |                |
| score    | int(11)      | NO   |     | NULL    |                |
+----------+--------------+------+-----+---------+----------------+
5 rows in set (0.00 sec)
```

- 查看创建表的SQL语句：SHOW CREATE TABLE <表名>

```
mysql> SHOW CREATE TABLE students;
+----------+-------------------------------------------------------+
| students | CREATE TABLE `students` (                             |
|          |   `id` bigint(20) NOT NULL AUTO_INCREMENT,            |
|          |   `class_id` bigint(20) NOT NULL,                     |
|          |   `name` varchar(100) NOT NULL,                       |
|          |   `gender` varchar(1) NOT NULL,                       |
|          |   `score` int(11) NOT NULL,                           |
|          |   PRIMARY KEY (`id`)                                  |
|          | ) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8 |
+----------+-------------------------------------------------------+
1 row in set (0.00 sec)
```

- 创建表使用语句：CREATE TABLE <表名>
- 删除表使用语句：DROP TABLE <表名>

```
mysql> DROP TABLE students;
Query OK, 0 rows affected (0.01 sec)
```

- 修改表

  添加列：（eg.给`students`表新增一列`birth`) 

  ALTER TABLE <表名> ADD COLUMN...

```
ALTER TABLE students ADD COLUMN birth VARCHAR(10) NOT NULL;
//添加的 birth 列为VARCHAR（10）类型（变长字符串0~10个字符）且不能为NULL				
```

​		修改`birth`列，例如把列名改为`birthday`，类型改为`VARCHAR(20)`：

​		ALTER TABLE <表名> CHANGE COLUMN ...

```
ALTER TABLE students CHANGE COLUMN birth birthday VARCHAR(20) NOT NULL;
```

​		删除列：

​		ALTER TABLE <表名>  DROP COLUMN <>字段名

```
ALTER TABLE students DROP COLUMN birthday;
```

#### **退出MySQL**

使用`EXIT`命令退出MySQL：

```
mysql> EXIT
Bye
```

注意`EXIT`仅仅断开了客户端和服务器的连接，MySQL服务器仍然继续运行。

### 6.2 实用SQL语句

- **插入或替换** 

使用 REPLACE 直接进行 查询（+删除）+插入。

如果我们希望插入一条新记录（INSERT），但如果记录已经存在，就先删除原记录，再插入新记录。此时，可以使用`REPLACE`语句，这样就不必先查询，再决定是否先删除再插入：

```
REPLACE INTO students (id, class_id, name, gender, score) VALUES (1, 1, '小明', 'F', 99);
```

若`id=1`的记录不存在，`REPLACE`语句将插入新记录，否则，当前`id=1`的记录将被删除，然后再插入新记录。

- **插入或更新** 

INSERT INTO ... ON DUPLICATE KEY UPDATE ...实现 插入/更新

如果我们希望插入一条新记录（INSERT），但如果记录已经存在，就更新该记录，此时，可以使用`INSERT INTO ... ON DUPLICATE KEY UPDATE ...`语句：

```
INSERT INTO students (id, class_id, name, gender, score) VALUES (1, 1, '小明', 'F', 99) ON DUPLICATE KEY UPDATE name='小明', gender='F', score=99;
```

若`id=1`的记录不存在，`INSERT`语句将插入新记录，否则，当前`id=1`的记录将被更新，更新的字段由`UPDATE`指定。

- **插入或忽略**

INSERT IGNORE INTO ... 实现 插入/忽略

如果我们希望插入一条新记录（INSERT），但如果记录已经存在，就啥事也不干直接忽略，此时，可以使用`INSERT IGNORE INTO ...`语句：

```
INSERT IGNORE INTO students (id, class_id, name, gender, score) VALUES (1, 1, '小明', 'F', 99);
```

若`id=1`的记录不存在，`INSERT`语句将插入新记录，否则，不执行任何操作。

- **快照**

CREATE TABLE <表名> SELECT ... 实现快照（将SELECT的结果复制到新表）

如果想要对一个表进行快照，即复制一份当前表的数据到一个新表，可以结合`CREATE TABLE`和`SELECT`：

```
-- 对class_id=1的记录进行快照，并存储为新表students_of_class1:
CREATE TABLE students_of_class1 SELECT * FROM students WHERE class_id=1;
```

新创建的表结构和`SELECT`使用的表结构完全一致。

- **写入查询结果集**

INSERT + SELECT

如果查询结果集需要写入到表中，可以结合`INSERT`和`SELECT`，将`SELECT`语句的结果集直接插入到指定表中。

例如，创建一个统计成绩的表`statistics`，记录各班的平均成绩：

```
CREATE TABLE statistics (
    id BIGINT NOT NULL AUTO_INCREMENT,
    class_id BIGINT NOT NULL,
    average DOUBLE NOT NULL,
    PRIMARY KEY (id)
);
```

然后，我们就可以用一条语句写入各班的平均成绩：

```
INSERT INTO statistics (class_id, average) SELECT class_id, AVG(score) FROM students GROUP BY class_id;
```

确保`INSERT`语句的列和`SELECT`语句的列能一一对应

```
> SELECT * FROM statistics;
+----+----------+--------------+
| id | class_id | average      |
+----+----------+--------------+
|  1 |        1 |         86.5 |
|  2 |        2 | 73.666666666 |
|  3 |        3 | 88.333333333 |
+----+----------+--------------+
3 rows in set (0.00 sec)
```

- **强制使用指定索引**

FORCE INDEX

在查询的时候，数据库系统会自动分析查询语句，并选择一个最合适的索引。但是很多时候，数据库系统的查询优化器并不一定总是能使用最优索引。如果我们知道如何选择索引，可以使用`FORCE INDEX`强制查询使用指定的索引。例如：

```
> SELECT * FROM students FORCE INDEX (idx_class_id) WHERE class_id = 1 ORDER BY id DESC;
```

指定索引的前提是索引`idx_class_id`必须存在。

## 7 事务

数据库事务：将多条语句作为一个整体进行操作的功能。

事务范围内的所有操作全部成功或者全部失败。如果事务失败，那么效果就和没有执行这些SQL一样，不会对数据库数据有任何改动。

特点：ACID

- A：Atomic，原子性，将所有SQL作为原子工作单元执行，要么全部执行，要么全部不执行；
- C：Consistent，一致性，事务完成后，所有数据的状态都是一致的，即A账户只要减去了100，B账户则必定加上了100；
- I：Isolation，隔离性，如果有多个事务并发执行，每个事务作出的修改必须与其他事务隔离；
- D：Duration，持久性，即事务完成后，对数据库数据的修改被持久化存储。

隐式事务：单条SQL语句

显式事务：BEGIN开启 + COMMIT提交/ROLLBACK回滚

```
BEGIN;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
UPDATE accounts SET balance = balance + 100 WHERE id = 2;
COMMIT;  //COMMIT试图把事务内的所有SQL所做的修改永久保存

BEGIN;
UPDATE accounts SET balance = balance - 100 WHERE id = 1;
UPDATE accounts SET balance = balance + 100 WHERE id = 2;
ROLLBACK;  //ROLLBACK回滚事务，整个事务会失败
```

**隔离级别：**( 解决并发操作导致的数据不一致性问题)

| Isolation Level  | 脏读（Dirty Read） | 不可重复读（Non Repeatable Read） | 幻读（Phantom Read） |
| :--------------- | :----------------- | :-------------------------------- | :------------------- |
| Read Uncommitted | Yes                | Yes                               | Yes                  |
| Read Committed   | -                  | Yes                               | Yes                  |
| Repeatable Read  | -                  | -                                 | Yes                  |
| Serializable     | -                  | -                                 | -                    |

```SQL
//设置隔离级别（注：MySQL的InnoDB默认隔离等级为Repeatable Read）
SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED;
```

### 7.1 Read Uncommitted

脏读：一个事务会读到另一个事务更新后但未提交的数据，如果另一个事务回滚，那么当前事务读到的数据就是脏数据。

示例：

| 时刻 | 事务A                                             | 事务B                                             |
| :--- | :------------------------------------------------ | :------------------------------------------------ |
| 1    | SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED; | SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED; |
| 2    | BEGIN;                                            | BEGIN;                                            |
| 3    | UPDATE students SET name = 'Bob' WHERE id = 1;    |                                                   |
| 4    |                                                   | SELECT * FROM students WHERE id = 1;              |
| 5    | ROLLBACK;                                         |                                                   |
| 6    |                                                   | SELECT * FROM students WHERE id = 1;              |
| 7    |                                                   | COMMIT;                                           |

### 7.2 Read Committed

不可重复读：在一个事务内，多次读同一数据，在这个事务还没有结束时，如果另一个事务恰好修改了这个数据，那么，在第一个事务中，两次读取的数据就可能不一致。

事务不可重复读同一条记录，因为很可能读到的结果不一致。

| 时刻 | 事务A                                           | 事务B                                           |
| :--- | :---------------------------------------------- | :---------------------------------------------- |
| 1    | SET TRANSACTION ISOLATION LEVEL READ COMMITTED; | SET TRANSACTION ISOLATION LEVEL READ COMMITTED; |
| 2    | BEGIN;                                          | BEGIN;                                          |
| 3    |                                                 | SELECT * FROM students WHERE id = 1;            |
| 4    | UPDATE students SET name = 'Bob' WHERE id = 1;  |                                                 |
| 5    | COMMIT;                                         |                                                 |
| 6    |                                                 | SELECT * FROM students WHERE id = 1;            |
| 7    |                                                 | COMMIT;                                         |

### 7.3 Repeatable Read

幻读：在一个事务中，第一次查询某条记录，发现没有，但是，当试图更新这条不存在的记录时，竟然能成功，并且，再次读取同一条记录，它就神奇地出现了。

注：与Read commited不同的是，即使另一个事务已经commit了新的记录，在该事务中直接SELECT 仍然是EMPTY的，但UPDATE一次后，就可以SELECT出来了。

（幻读就是没有读到的记录，以为不存在，但其实是可以更新成功的，并且，更新成功后，再次读取，就出现了。）

| 时刻 | 事务A                                               | 事务B                                             |
| :--- | :-------------------------------------------------- | :------------------------------------------------ |
| 1    | SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;    | SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;  |
| 2    | BEGIN;                                              | BEGIN;                                            |
| 3    |                                                     | SELECT * FROM students WHERE id = 99;             |
| 4    | INSERT INTO students (id, name) VALUES (99, 'Bob'); |                                                   |
| 5    | COMMIT;                                             |                                                   |
| 6    |                                                     | SELECT * FROM students WHERE id = 99;             |
| 7    |                                                     | UPDATE students SET name = 'Alice' WHERE id = 99; |
| 8    |                                                     | SELECT * FROM students WHERE id = 99;             |
| 9    |                                                     | COMMIT;                                           |

### 7.4 Serializable

最严格的隔离级别，事务串行执行，但效率会大大下降，故如果没有特别重要的情景，一般都不会使用Serializable隔离级别。

**默认隔离级别：**

在MySQL中，如果使用InnoDB，默认的隔离级别是Repeatable Read。

## 8 开发集成（无）

无

## 9 期末总结（无）

无

## 10 备注

1. 所有MySQL命令要以`;`结尾

2.  字符串要用`''`括起来（例如下例中的'lyf','female'）

   ```SQL
   INSERT INTO students (class_id,name,gender) VALUES(1,'lyf','female');
   ```


## 11 postgreSQL 管理命令

查看已有数据库：\l

创建数据库：CREATE DATABASE dbname;

进入数据库：\c + 数据库名

查看已有表格：\d

创建表格：

```
//例子：（注意没有AUTO_INCREMENT，表格名字也不能有单引号''）
CREATE TABLE COMPANY(
   ID INT PRIMARY KEY     NOT NULL,
   NAME           TEXT    NOT NULL,
   AGE            INT     NOT NULL,
   ADDRESS        CHAR(50),
   SALARY         REAL
);
```

删除表格：DROP TABLE table_name;

# III 改写笔记

#### std::path::Path

- `pub fn display(&self) -> Display`

打印 Path

Returns an object that implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) for safely printing paths that may contain non-Unicode data.

```rust
use std::path::Path;

let path = Path::new("/tmp/foo.rs");

println!("{}", path.display());
```

- pub fn [is_file](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file)(&self) -> [bool](https://doc.rust-lang.org/std/primitive.bool.html)

Returns `true` if the path exists on disk and is pointing at a regular file.

- pub fn [is_dir](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_dir)(&self) -> [bool](https://doc.rust-lang.org/std/primitive.bool.html)

Returns `true` if the path exists on disk and is pointing at a directory.

#### mysql (version:17.0.0)

**Struct Pool**

- `pub fn prepare<T: AsRef<str>>(&self, query: T) -> MyResult<Stmt<'static>>`[[src\]](https://docs.rs/mysql/17.0.0/src/mysql/conn/pool.rs.html#242-245)[[−\]](javascript:void(0))

  Will prepare statement. See [`Conn::prepare`](https://docs.rs/mysql/17.0.0/mysql/struct.Conn.html#method.prepare).

  It will try to find connection which has this statement cached.

- `pub fn prep_exec<A, T>(  &self,  query: A,  params: T) -> MyResult<QueryResult<'static>>where  A: AsRef<str>,  T: Into<Params>, `[[src\]](https://docs.rs/mysql/17.0.0/src/mysql/conn/pool.rs.html#251-269)[[−\]](javascript:void(0))

  Shortcut for `pool.get_conn()?.prep_exec(..)`. See [`Conn::prep_exec`](https://docs.rs/mysql/17.0.0/mysql/struct.Conn.html#method.prep_exec).

  It will try to find connection which has this statement cached.

**Struct Conn**

- `pub fn prepare<T: AsRef<str>>(&mut self, query: T) -> MyResult<Stmt>`[[src\]](https://docs.rs/mysql/17.0.0/src/mysql/conn/mod.rs.html#948-955)[[−\]](javascript:void(0))

  Implements binary protocol of mysql server.

  Prepares mysql statement on `Conn`. [`Stmt`](https://docs.rs/mysql/17.0.0/mysql/struct.Stmt.html) will borrow `Conn` until the end of its scope.

  This call will take statement from cache if has been prepared on this connection.

- `pub fn prep_exec<A, T>(&mut self, query: A, params: T) -> MyResult<QueryResult>where  A: AsRef<str>,  T: Into<Params>, `[[src\]](https://docs.rs/mysql/17.0.0/src/mysql/conn/mod.rs.html#961-967)[[−\]](javascript:void(0))

  Prepares and executes statement in one call. See ['Conn::prepare'](https://docs.rs/mysql/17.0.0/mysql/struct.Conn.html#method.prepare)

str -> stmt -> prepare -> execuate

**Struct Vec**

- `pub fn resize(&mut self, new_len: usize, value: T)`1.5.0[[src\]](https://doc.rust-lang.org/src/alloc/vec.rs.html#1555-1563)[[−\]](javascript:void(0))

  Resizes the `Vec` in-place so that `len` is equal to `new_len`.

  If `new_len` is greater than `len`, the `Vec` is extended by the difference, with each additional slot filled with `value`. If `new_len` is less than `len`, the `Vec` is simply truncated.

**std::thread::sleep**

对应 `Thread.sleep(interval);`

```rust
use std::{thread, time};

let ten_millis = time::Duration::from_millis(10);
let now = time::Instant::now();

thread::sleep(ten_millis);

assert!(now.elapsed() >= ten_millis);
```



#### 其他

**python中的字符串前缀 r,b,u**

1. r'字符串'
       r应该是raw的缩写，表示不需要加工，仅仅包裹一串字符串。例如：r'\n'，不表示换行，仅仅表示反斜杠'\'和小写字母'n'组成的字符串。不需要加工、转义等进行翻译。

2. b'字符串'
       b表示bytes的意思，即要求程序加字符串加载为bytes类型，bytes类型的数据常用于网络的数据封装。

    在 Python3 中，bytes 和 str 的互相转换方式是
   
    str.encode('utf-8')
   
    bytes.decode('utf-8')

3. u'字符串' 
       u便是Unicode，u前缀的字符串表示以Unicode编码，一般将包含中文的字符串加上u前缀，以避免错误。    

# IV 改写的问题

### Encoder 部分

- File shardFile = new File(shardsFolder, Integer.toString(fid * 100 + i));

方法`java.io.File.File(File parent, String child)` 找不到对应的rust 方法。不确定是否是在shardsFolder下创建了child目录

- long java.io.File.length()

  所述 **java.io.File.length（）** 返回此抽象路径名定义的文件的长度。如果此路径名定义目录，则未指定返回值。

```java
File outputFile = new File(shardsFolder, Integer.toString(fid * 100 + i));
OutputStream out = new FileOutputStream(outputFile);
out.write(shards[i]);
out.close();
```

```rust
let path:Path = *shardsFolder.join(Path::new(&(fid * 100 +i).to_string())).as_path();
//还有些问题
```

- **struct Path** ->`pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf`[[src\]](https://doc.rust-lang.org/src/std/path.rs.html#2178-2180)[[−\]](javascript:void(0))   

  Creates an owned [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html) with `path` adjoined to `self`.

  See [`PathBuf::push`](https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push) for more details on what it means to adjoin a path.

- **struct PathBuf** -> `pub fn as_path(&self) -> &Path`[[src\]](https://doc.rust-lang.org/src/std/path.rs.html#1150-1152)[[−\]](javascript:void(0))

  Coerces to a [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) slice.

### 注意事项

- FileUtil

  由于没有函数重载，故函数名加了 _str

  ```java
  //java   
  public static void clearFolder(String folderPath) {
          clearFolder(new File(folderPath));
      }
  ```

  ```rust
  //rust
  pub fn clearFolder_str(&self,folderPath:String) {
               self.clearFolder(folderPath.try_into().unwrap());
           }
  ```

- FolderScanner

  ```java
  //java
  if (file.canRead()) {
  			attribute = attribute + 'r';
  		} else {
  			attribute = attribute + '-';
  		}
  		if (file.canWrite()) {	//这部分没法实现
  			attribute = attribute + 'w';
  		} else {
  			attribute = attribute + '-';
  		}
  ```

  ```rust
  //rust
  let metadata = file.metadata().unwrap();
           if metadata.permissions().readonly() == true {
               attribute = attribute + "r";
           } else {
               attribute = attribute + "-";
           }
  ```

  file、path、pathbuf 获取文件权限都要用到

  `pub fn permissions(&self) -> Permissions`[[src\]](https://doc.rust-lang.org/src/std/fs.rs.html#1070-1072)[[−\]](javascript:void(0))

  Returns the permissions of the file this metadata is for.

  而struct std::fs::Permissions

  > Representation of the various permissions on a file.

  > This module only currently provides one bit of information, [`readonly`](https://doc.rust-lang.org/std/fs/struct.Permissions.html#method.readonly), which is exposed on all currently supported platforms. Unix-specific functionality, such as mode bits, is available through the [`PermissionsExt`](https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html) trait.
  
  目前在windows上只实现了查看文件权限是否为只读

# V 进度记录

### SQL示例程序

- MySQL

  我在pc上安装的 mysql 版本为`mysql-8.0.20-winx64`，安装教程与 zip 安装的常见问题解决帖 在 II SQL笔记 2安装MySQL 中。

  ```
  //cargo.toml中crate mysql的版本
  [dependencies]
  mysql = "17.0.0"
  ```

  ```rust
  //官方示例程序
  #[macro_use]
  extern crate mysql;
  // ...
  
  use mysql as my;
  
  #[derive(Debug, PartialEq, Eq)]
  struct Payment {
      customer_id: i32,
      amount: i32,
      account_name: Option<String>,
  }
  
  
  fn main() {
      // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
      let pool = my::Pool::new("mysql://root:password@localhost:3306/mysql").unwrap();
  
      // Let's create payment table.
      // Unwrap just to make sure no error happened.
      //！示例程序给的是TEMPORARY TABLE，但由于会报错找不到这张表，所以我去掉了TEMPORARY
      // pool.prep_exec(r"CREATE TEMPORARY TABLE payment (
      //                      customer_id int not null,
      //                      amount int not null,
      //                      account_name text
      //                  )", ()).unwrap();
  
      pool.prep_exec(r"CREATE TABLE payment (
          customer_id int not null,
          amount int not null,
          account_name text
      )", ()).unwrap();
      //r""是raw的缩写，表示不需要加工，仅仅包裹一串字符串。
  
      let payments = vec![
          Payment { customer_id: 1, amount: 2, account_name: None },
          Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
          Payment { customer_id: 5, amount: 6, account_name: None },
          Payment { customer_id: 7, amount: 8, account_name: None },
          Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
      ];
  
      // Let's insert payments to the database
      // We will use into_iter() because we do not need to map Stmt to anything else.
      // Also we assume that no error happened in `prepare`.
      for mut stmt in pool.prepare(r"INSERT INTO payment
                                         (customer_id, amount, account_name)
                                     VALUES
                                         (:customer_id, :amount, :account_name)").into_iter() {
     //外层for循环只循环一次
          for p in payments.iter() {
              // `execute` takes ownership of `params` so we pass account name by reference.
              // Unwrap each result just to make sure no errors happened.
              stmt.execute(params!{
                  "customer_id" => p.customer_id,
                  "amount" => p.amount,
                  "account_name" => &p.account_name,
              }).unwrap();
          }
      }
  
      // Let's select payments from database
      let selected_payments: Vec<Payment> =
      pool.prep_exec("SELECT customer_id, amount, account_name from payment", ())
      .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
          // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
          // will map each `MyResult` to contained `row` (no proper error handling)
          // and second call to `map` will map each `row` to `Payment`
          result.map(|x| x.unwrap()).map(|row| {
              // ⚠️ Note that from_row will panic if you don't follow your schema
              let (customer_id, amount, account_name) = my::from_row(row);
              Payment {
                  customer_id: customer_id,
                  amount: amount,
                  account_name: account_name,
              }
          }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
      }).unwrap(); // Unwrap `Vec<Payment>`
  
      // Now make sure that `payments` equals to `selected_payments`.
      // Mysql gives no guaranties on order of returned rows without `ORDER BY`
      // so assume we are lukky.
      assert_eq!(payments, selected_payments);
      println!("Yay!");
  }
  ```
```
  
```
      let pool = my::Pool::new("mysql://root:password@localhost:3306/mysql").unwrap();
```
  
- 端口号
  
  我pc上的mysql端口号是3306，但安装不同版本的mysql默认端口号可能不同，也可自定义。
  
- root是用户名（mysql默认的用户名） password替换成密码
  
    例：如果密码是`ABCDEF` ，直接替换即可 -> "mysql://root:ABCDEF@localhost:3306/mysql"
    
  - 若运行成功，则打印 “Yay!”
  
- postgreSQL
  
    我在pc上安装的postgreSQL版本为`postgresql-12.3-1-windows-x64` 
  
  windows上的安装教程：https://www.runoob.com/postgresql/windows-install-postgresql.html
  
```
  //cargo.toml中crate postgres的版本
  [dependencies]
  postgres = "0.15.2"
  ```
  
  ```rust
  extern crate postgres;
  
  use postgres::{Connection, TlsMode};
  
  struct Person {
      id: i32,
      name: String,
      data: Option<Vec<u8>>
  }
  
  fn main() {
      let conn = Connection::connect("postgresql://postgres:password@localhost:5432", TlsMode::None)
              .unwrap();
  
      conn.execute("CREATE TABLE person (
                      id              SERIAL PRIMARY KEY,
                      name            VARCHAR NOT NULL,
                      data            BYTEA
                    )", &[]).unwrap();
      let me = Person {
          id: 0,
          name: "Steven".to_owned(),
          data: None
      };
      conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                   &[&me.name, &me.data]).unwrap();
  
      for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
          let person = Person {
              id: row.get(0),
              name: row.get(1),
              data: row.get(2)
          };
          println!("Found person {}", person.name);
      }
  }
  ```

  ```rust
      let conn = Connection::connect("postgresql://postgres[:password]@localhost:5432", TlsMode::None)
  ```

  - 端口号
  
    我pc上的postgreSQL端口号是5432，但安装不同版本的postgreSQL默认端口号可能不同（eg. 5433），也可自定义。
  
  - postgresql是用户名（postgreSQLl默认的用户名） password替换成密码
  
    例：如果密码是`ABCDEF` ，去掉`[]`替换即可 -> "postgresql://postgres:ABCDEF@localhost:5432"
  
  - 若运行成功，则打印“Found person Steven”
  
- cargo run时可能出现的问题

  ```
  //数据库未启动 或 端口号错误
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Io(Os { code: 10061, kind: ConnectionRefused, message: "由于目标计算机积极拒绝，无法连接。" }))', src\main.rs:12:16
  ```

  出现错误时有note提示如下：

  ​	note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  设置方法：

  在cmd中执行 set RUST_BACKTRACE=1

  在powershell中执行 $Env:RUST_BACKTRACE=1

  设置之后，若程序错误运行，则会打印类似如下信息：

  ```
     1: std::sys_common::backtrace::_print_fmt
               at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447\/src\libstd\sys_common\backtrace.rs:77
     2: std::sys_common::backtrace::_print::{{impl}}::fmt
               at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447\/src\libstd\sys_common\backtrace.rs:59
     3: core::fmt::write
               at /rustc/b8cedc00407a4c56a3bda1ed605c6fc166655447\/src\libcore\fmt\mod.rs:105
  ```

  （我看不太懂，感觉没什么用）
  
### Encoder Decoder

- 解决了二维数组的问题，用Vec<Vec<u8>>解决，但外层“数组索引”编译报错

解决：使用 as 强制类型

- 还有一直存在的Path的使用问题

解决：统一将原java使用File的地方全部替换成了PathBuf

  ### Rust 代码结构构建

- crate根
  - client
    - client
      - client.rs	//main函数在这里
      - SynItem.rs
    - com
      - Encoder.rs
      - Decoder.rs
      - 纠删码的crate
    - connect
      - FileTransporter.rs
      - FragmentManager.rs
      - ServerConnect.rs
    - fileDetector
      - FileAttrs.rs	mod
      - FileUploader.rs  
      - FileUtil.rs
      - FolderScanner.rs
  - server

讨论结果：

直接cargo run即可，做成二进制可执行文件

client、server各有main，做成两个包这样，然后直接cargo run

目前Bug较多，跑不通

### Debug

**6/24**

1. 锁机制的一些问题
![](C:\Users\12935\Pictures\Screenshots\批注 2020-06-24 164353.png)

![](C:\Users\12935\Pictures\Screenshots\批注 2020-06-24 164412.png)

2. static

   很多 init 方法是 static 类方法，改变的是static 类变量，但在改写过程中，都是当普通成员方法，成员变量来写。

3. 参数带不带引用的问题

