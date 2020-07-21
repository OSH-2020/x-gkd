进度：编译成功 server，调试代码中



# Mysql

Server

database

Query.rs

数据库中具体包括表 FILE 用于存储文件的逻辑位置与属性、表 FRAGMENT 用于存储碎片的物理位置、表 REQUEST 用于存储服务器对客户端的碎片请求、表 DEVICE 用于存储系统中客户端的信息、表 USER 用于存储网页的注册用户

Opts 此结构包含服务器主机名，客户端用户名/密码和其他设置，这些设置控制客户端的行为。

​	OptsBuilder

Conn 此结构表示一个活动的MySql连接

Pool 它是对连接池的引用，可以在线程之间克隆和共享连接池。

Statement 语句只是与语句元数据结合的标识符，即有关其参数和列的信息。

Value 此枚举表示MySql单元格的原始值

![image-20200519104841137](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200519104841137.png)

MySql本身没有命名参数支持，因此在客户端实现。应该将其`:name`用作命名参数的占位符语法。

命名参数可以在语句内重复，例如`SELECT :foo, :foo`将需要单个命名参数`foo`，该参数将在语句执行期间在相应位置重复。

应该使用`params!`宏来构建执行参数。

## 查询

```rust
    // Let's select payments from database
    let selected_payments: Vec<Payment> =
    pool.prep_exec("SELECT customer_id, amount, account_name from tmp.payment", ())
    .map(|result| { // In this closure we sill map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|mut row| {
            Payment {
                account_name: from_value(row.pop().unwrap()),
                amount: from_value(row.pop().unwrap()),
                customer_id: from_value(row.pop().unwrap()),
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`
```

```rust
// Let's select payments from database. Type inference should do the trick here.
let selected_payments = conn
    .query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    )?;
```

    let result = conn.prep_exec("SELECT username, email FROM user where id = :id", params! {
        "id" => 1
    }).await?;
    
    let (_, user_info) = result.map_and_drop(|row| {
        let (username, email): (Option<String>, Option<String>) = mysql_async::from_row(row);
    
        (username, email)
    }).await?;
    
    //
    let result = conn.prep_exec("SELECT username, email FROM user where id = :id", params! {
        "id" => 1
    }).await?;
    
    let (_, user_info) = result.map_and_drop(|row| {
        let username: Option<String> = row.get("username");
        let email: Option<String> = row.get("email");
    
        (username, email)
    }).await?;
## 添加

```
// Now let's insert payments to the database
conn.exec_batch(
    r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
    payments.iter().map(|p| params! {
        "customer_id" => p.customer_id,
        "amount" => p.amount,
        "account_name" => &p.account_name,
    })
)?;

// Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO tmp.payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (?, ?, ?)").into_iter() {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happended.
            stmt.execute((p.customer_id, p.amount, &p.account_name)).unwrap();
        }
    }
```

所述`Queryable`性状定义了常用方法`Conn`，`PooledConn`和`Transaction`。基本方法const的集合：

- `query_iter`-执行文本查询和获取的基本方法`QueryRestul`；
- `prep` -准备陈述的基本方法；
- `exec_iter`-执行语句并获取的基本方法`QueryResult`；
- `close` -关闭语句的基本方法；

该特征还定义了基于基本方法的一组辅助方法。这些方法将仅使用firt结果集，而其他结果集将被丢弃：

- `{query|exec}`-将结果收集到中`Vec`；
- `{query|exec}_first`-获得第一个`T: FromRow`（如果有）；
- `{query|exec}_map`-将每个映射`T: FromRow`到一些`U`;
- `{query|exec}_fold`-将的集合折叠`T: FromRow`为单个值；
- `{query|exec}_drop` -立即删除结果。

该特征还定义了`exec_batch`函数，它是批处理语句执行的助手。



snake case

getInt get_int



Queryable.rs

```
/// Performs text query and drops the query result.
    fn query_drop<Q>(&mut self, query: Q) -> Result<()>
    where
        Q: AsRef<str>,
    {
        self.query_iter(query).map(drop)
    }
    
/// Prepares the given statement, and exectues it with each item in the given params iterator.
    fn exec_batch<S, P, I>(&mut self, stmt: S, params: I) -> Result<()>
    where
        Self: Sized,
        S: AsStatement,
        P: Into<Params>,
        I: IntoIterator<Item = P>,
    {
        let stmt = stmt.as_statement(self)?;
        for params in params {
            self.exec_drop(stmt.as_ref(), params)?;
        }

        Ok(())
    }

// Now let's insert payments to the database
conn.exec_batch(
    r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
    payments.iter().map(|p| params! {
        "customer_id" => p.customer_id,
        "amount" => p.amount,
        "account_name" => &p.account_name,
    })
)?;
```

调用另一个文件？

Query/FileItem.rs

use ...  pub use有传递性

Query 文件夹，FileItem.rs文件



## MySQL 详细设计报告

第一组主要负责实现客户端java和服务器java，并实现两者间的通讯和文件传输； 另一组实现了web服务和网页的实现，搭建了一个网站。 两组之间的融合靠的是mysql数据库，二者通过数据库传达各个子系统的状态信息和请求。当整个系统出错时，我们通过数据库这个中间层的信息就可以初步判断错误出现在哪一组。 

每个 ClientThread 类的实例负责与一台客户机的控制报文通信，其不断从 socket 的输出流读取客户端发来的报文并判断报文的种类）： 

（一）若是客户端状态报文，其根据报文内容在数据库中更新状态，然后查询等待这个客户端处理的请求数量并装在回复报文中发送给客户端； 

（二）若是处理请求报文，其在数据库中查找等待该客户端处理的第一条请求并返回给客户端； 

（三）若是中断连接报文，其立即中断链接并在数据库中记录该客户端已经下线； 

以下结合使用场景例举几个报文的处理过程： 

（1）服务器请求文件碎片：

- 在数据库的 REQUEST 表中写入请求 
- 客户端在下次心跳连接时将收到请求 
- 客户端将新建一条与服务器的数据连接并将碎片发往服务器,发送后客户端上的碎片将不会被删除.
- 服务器收到碎片后将会把碎片保存到下载文件夹并删除数据库中的请求 

（2）文件上传: 

- 客户端需要向服务器发送上传文件报文 
- 服务器收到报文后将会在数据库 FILE 中插入文件项并返回文件的 ID.为了表示文件仍不可用,文件项的 NOA 字段将被设置为实际 NOA 的相反数 
- 客户端接下来向服务器发送文件的各个碎片,其中最后一个碎片一定要最后一个发送,之前的碎片可以乱序发送. 
- 服务器收到碎片后将会把碎片保存在本地临时文件夹并数据库 FRAGMENT 中插入碎片项.由于碎片未被分配到文件夹,此时 PATH 均为'-1' 
- 服务器收到最后一个碎片后检测是否碎片已被全部上传,如是,将碎片分配到各个客户端并将数据库中文件项的 NOA 字段改为实际的碎片数量.分配的方法是在数据库 REQUEST 中写入请求等待客户端取走碎片 
- 被分配到的客户端在下次心跳连接时将收到请求 
- 客户端将新建一条与服务器的数据连接并接收碎片,发送完碎片后服务器将暂存的碎片删除,同时将数据库中碎片项的 PATH 改为设备的 ID 
- 客户端收到碎片后将会把碎片保存到碎片文件夹,服务器完成传输后删除数据库中的请求 

（3）碎片删除：

- 在数据库 REQUEST 中写入请求 
- 客户端在下次心跳连接时将收到请求 
- 客户端将删除这一碎片并新建一条与服务器的数据连接以通知服务器请求完成. 
- 服务器通知后删除数据库中的请求 

### 数据库表定义：

```
CREATE TABLE `DEVICE` (   
`ID` int NOT NULL AUTO_INCREMENT,   
`IP` char(20) NOT NULL DEFAULT '',   
`PORT` int NOT NULL DEFAULT 0,   
`ISONLINE` boolean NOT NULL,   
`RS` int NULL DEFAULT 0 ,   
PRIMARY KEY (`ID`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8; 
 
CREATE TABLE `FRAGMENT` (   
`ID` int NOT NULL,   
`PATH` char(20) NOT NULL DEFAULT '',   
PRIMARY KEY (`ID`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8; 
 
CREATE TABLE `FILE` (   
`ID` int NOT NULL AUTO_INCREMENT,   
`NAME` char(20) NOT NULL DEFAULT '',   
`PATH` char(60) NOT NULL DEFAULT '',   
`ATTRIBUTE` char(10) NOT NULL DEFAULT '',   
`TIME` char(10) NOT NULL DEFAULT '',   
`NOA` int NOT NULL DEFAULT 1,   
`ISFOLDER` boolean NOT NULL DEFAULT false,   
PRIMARY KEY (`id`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8; 
 
CREATE TABLE `REQUEST` (   
`ID` int NOT NULL AUTO_INCREMENT,   
`TYPE` int NOT NULL DEFAULT 0,   
`FRAGMENTID` int NOT NULL DEFAULT 0,   
`DEVICEID` int NOT NULL DEFAULT 0,   
PRIMARY KEY (`ID`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8; 
 
CREATE TABLE `USER` (   
`ID` int NOT NULL AUTO_INCREMENT,   
`NAME` char(20) NOT NULL UNIQUE DEFAULT '', 
`PASSWD` char(20) NOT NULL DEFAULT '',   
PRIMARY KEY (`ID`) 
) ENGINE=InnoDB DEFAULT CHARSET=utf8; 
```

Query 类定义了对上述五个表查询、修改、删除、新增条目的函数，其通过 JDBC 接口实现了对数据的访问，访问的流程为： 

（一）在构造函数中使用 DriverManager.getConnection 函数创建到数据库的连接（一个Connection 类实例）； 

（二）通过 Connection 类实例的 createStatement 函数创建一个 Statement 类实例； 

（三）通过 Statement 类实例的 executeQuery 函数执行 SQL，SQL 的内容可以使用格式化字符串根据函数的参数填入不同的内容，该函数将返回一个 ResultSet 类实例； 

（四）对 ResultSet 类实例，使用 next 函数与 getInt、getBoolean、getString 等函数遍历查询的每个结果； 

（五）对 ResultSet 类实例与 Statement 类实例，执行 close 函数关闭连接； 

（六）在 closeConnection 函数中，调用 Connection 类实例 close 函数关闭连接。



版本问题 各个版本之间的方法不一样

## Result

![image-20200610200653631](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200610200653631.png)

Result 是一个函数返回的类型，它可以是 Err，也可以是 Ok。如果是 Ok，则表示函数按照预期执行完成。如果是 Err，则该函数出现了错误。

![image-20200610201110578](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200610201110578.png)

转换后的数字可以可以通过在后面附加调用 *unwrap()* 函数将数字从 Result 中“解压”出来，*unwrap()* 函数可以看出 Result 中类型，可能是 *Ok*，也可能是 *Err*。如果 Result 中包裹的类型是 *Ok*，那么 *unwrap()* 则返回它的值。如果 Result 中的类型是 *Err*，*unwrap()* 则会让程序崩溃。

![image-20200610202236422](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200610202236422.png)

![image-20200610202108808](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200610202108808.png)



## 安装 MySQL 遇到的问题

大部分在 lyf.md 中有，补充两条

.err 文档中有 Do you already have another mysqld server running on port: 3306 ?

https://blog.csdn.net/weixin_43250455/article/details/88372731

netstat -ano

taskkill /f /pid 3312

登陆之后 ERROR 1820 (HY000): You must reset your password using ALTER USER statement before executing this statement.

https://blog.csdn.net/muziljx/article/details/81541896



## 基本 MYSQL 使用

net start mysql

mysql -u root -p

USE <新数据库>	USE DFS;

SHOW TABLES;

DESC <表>

SELECT * FROM DFS.FILE;

DELETE FROM DFS.



## 使用的实例程序

```
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
    let pool = my::Pool::new("mysql://root:mysql@localhost:3306/mysql").unwrap();

    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(r"CREATE TABLE payment (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )", ()).unwrap();

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
            //  Note that from_row will panic if you don't follow your schema
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

# 改写过程中的问题：

## Query：

getInt(index)，getString(index) 表明返回第 index 列的数据

rs.next() 下一行，最开始指针指第一行的上一行，所以必须先next才能开始读取。

count(*)对行的数目进行计算，包含NULL
count(column)对特定的列的值具有的行数进行计算，不包含NULL值。

use std::*;

include!("FileItem.rs");	使用宏来 include，可以不在同一目录，写清目录就可

rust 不支持函数重载，所以有些函数修改了名字，在后面加上 _Byid 等

和其他代码互联：

错误处理：如果查询错误，返回的结构体中ip值为-1

还没有在pool中添加user和password



## FileUploader：

1. int = Integer.parseInt(String) 表示把 String 的数字提取出来变成int

   可以使用rust中的

```
let my_string = "27".to_string(); // `parse()` works with `&str` and `String`!

let my_int = my_string.parse::<i32>().unwrap();
```

2. match &self.to_server{

   ​      None => println!("Error! server not connected..."),

   ​      Some (socket) => {

   ​        //self.socket = socket.clone();	后面是&tcpstream，前面是tcpstream

   ​      }

   ​    }	solved try_clone().unwrap()
   
3. java 中使用 File(String parent, String child); 创建新的文件：

   File f = new File(tmpFragmentFolder, Integer.toString(

   ​          fileId * 100 + fragmentNum));

   rust 中 tmpFragmentFolder 是 PathBuf，使用了 

   PathBuf ->(as_path)-> Path ->(File::create) -> File

   但其中有一点差别是 File::open 是要以只读方式打开，而且要是没有文件会报错，而 File::create 是以只写方式打开，要是没有文件创建文件。



## server：

架构：

main.rs

server.rs

server

- controlConnect.rs
- database.rs
- dataConnect.rs
- controlConnect
  - ClientThread.rs
  - ServerThread.rs
- database
  - DeviceItem.rs
  - FileItem.rs
  - RequestItem.rs
  - Query.rs
- dataConnect
  - ServerThread.rs
  - ClientThread.rs
  - FileTransporter.rs



1. self.socket.write_fmt(format_args!("{} false\n", fa));	不能把PathBuf输出

   ​	solved by fa.as_path().display()

2. 强制类型转换：

   ​	try_into()

   ​	xxx as i32	xxx as usize

3. an `extern crate` loading macros must be at the crate root

   ​	#[macro_use]

   ​	extern crate mysql;

   ​	外部依赖库放在 main.rs 中

4. immutable

   let request = query.queryFirstRequest_Byid(id);
      |                         ------- help: consider changing this to be mutable: `mut request`
   ...
   71 |                         request.get_id(), request.get_fragment_id(), request.get_type()));
      |                                                                      ^^^^^^^ cannot borrow as mutable

   

   pub fn alterFile(&self,file:FileItem) -> i32{
       |                            ---- help: consider changing this to be mutable: `mut file`
   ...
   478 |                     "name" => file.get_name(),
       |                               ^^^^ cannot borrow as mutable

5. java 里的字符串加法，使用了 rust String 中的 push_str 代替

   pub fn [push_str](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str)(&mut self, string: &[str](https://doc.rust-lang.org/std/primitive.str.html))

   pub fn [insert_str](https://doc.rust-lang.org/std/string/struct.String.html#method.insert_str)(&mut self, idx: [usize](https://doc.rust-lang.org/std/primitive.usize.html), string: &[str](https://doc.rust-lang.org/std/primitive.str.html))

6. unsafe 模块：string 到 vec， 使用了 as_mut_vec()，unsafe，需要

```
	unsafe {
        ipt = input.clone().as_mut_vec().to_vec();
    }
```

7. rust 常用类型转换

   | x类型\至类型 | i32                | u32  | f64  | String      |
   | ------------ | ------------------ | ---- | ---- | ----------- |
   | i32          |                    | as   | as   | to_string() |
   | u32          | as                 |      | as   |             |
   | f64          | as                 | as   |      |             |
   | String       | x.parse().unwrap() | ..   | ..   |             |

   string->&str	&*x

   &str->String	to_string()



## debug 中的问题：

1. rust 读取文件时好像会自己把 \ 变成两个 \

   所以 setup.ini 中不用写成两个 \
   
2. 貌似 FileUploader 中写的分几次发送实际上是一次发送的，所以要是在前一个报文后加 \n 后就看不到下一次报文了，所以删去换行符就可以把报文全部接收。（还没有修改完其他部分）

3. 增加 Query 中查询正确但没有符合要求的结果的情况：返回 id 为0.

4. 在 clearFolder 中添加 remove_dir，删去文件夹，rust 中还有 remove_dir_all

   | [remove_dir](https://doc.rust-lang.org/std/fs/fn.remove_dir.html) | Removes an existing, empty directory.                        |
   | ------------------------------------------------------------ | ------------------------------------------------------------ |
   | [remove_dir_all](https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html) | Removes a directory at this path, after removing all its contents. Use carefully! |

5. java 中 socOut.write(sendBytes, 0, length);

   在 server 和 client 端的 send_file 中把发送的数组长度改为 

   soc_out.write(&mut send_bytes[0..length as usize]);

   就可以解决发送文件大小变大的问题。

6. read_to_end eof -> write([0..len])

7. thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: UnexpectedEof, error: "failed to fill whole buffer" }', src\client\connect\FileTransporter.rs:10:5
   note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

   清空数据库中的信息和文件夹中的文件等

8. tmp 中的文件没有删掉，可能是文件属性为 只读 的问题

   solved：删去 set_readonly

9. alterDevice出错，注意到 IP 为 String 型，加双引号解决

   mysql> UPDATE DEVICE SET IP = "127.0.0.1" WHERE ID = 1;
   Query OK, 1 row affected (0.33 sec)
   Rows matched: 1  Changed: 1  Warnings: 0

   mysql> SELECT * FROM DFS.DEVICE;
   +----+-----------+------+----------+------+
   | ID | IP        | PORT | ISONLINE | RS   |
   +----+-----------+------+----------+------+
   |  1 | 127.0.0.1 |    0 |        1 |    0 |
   +----+-----------+------+----------+------+
   1 row in set (0.01 sec)

10. 有时候会 panic，可能是断开了连接，加了错误处理：如果断开之后，就返回错误，重新建立连接，再发送文件

    ![image-20200708210525431](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200708210525431.png)

11. java 中 setSoTimeout(5000); 是表示如果对方连接状态5秒没有收到数据的话强制断开客户端。

    rust 中找到 set_write_timeout 和 set_write_timeout，函数原型是：

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()>

    Duration：let five_seconds = Duration::new(5, 0); 每个持续时间由整数秒和以纳秒表示的小数部分组成。

12. keepalive 不是说TCP的长连接，当我们作为服务端，一个客户端连接上来，如果设置了keeplive为 true，当对方没有发送任何数据过来，超过一个时间(看系统内核参数配置)，那么我们这边会发送一个ack探测包发到对方，探测双方的TCP/IP连接是否有效(对方可能断点，断网)。
    如果不设置，那么客户端宕机时，服务器永远也不知道客户端宕机了，仍然保存这个失效的连接。

13. 测试3个客户端同时运行成功了，一个客户端检测两个文件夹成功了，可以传6M大小图片，decode成功

## 调用关系图：

<img src="./photo/image-20200705194021399.png" alt="image-20200705194021399" style="zoom:200%;" /><img src="./photo/image-20200705193719943.png" alt="image-20200705193719943" style="zoom:200%;" />

# 系统使用指南：

1. 首先在 client 中的 client.rs 中修改 ini 的路径：

   let setUpFile = String::from("C:\\Users\\sym\\projects\\setup.ini");

   把括号中的路径名改成自己的。

2. 而后按照 setup.ini 中的文件路径创建文件夹

   127.0.0.1
   6666
   6668
   1

   

   D:\mine\homework\file\fragment
   D:\mine\homework\file\tmp
   1
   D:\mine\homework\file\file1
   TIM

3. Query.rs 的文件中需要修改自己的 mysql 密码（xxxx处）

   let pool = my::Pool::new("mysql://root:xxxx@localhost:3306/server_test").unwrap();

4. 打开 mysql，创建新的数据库和表单：

   CREATE DATABASE DFS;

   USE DFS;

   CREATE TABLE `DEVICE` (  
   `ID` int NOT NULL AUTO_INCREMENT,  
   `IP` char(20) NOT NULL DEFAULT '',  
   `PORT` int NOT NULL DEFAULT 0,  
   `ISONLINE` boolean NOT NULL,  
   `RS` int NULL DEFAULT 0 ,  
   PRIMARY KEY (`ID`) 
   ) ENGINE=InnoDB DEFAULT CHARSET=utf8; 

   CREATE TABLE `FRAGMENT` (  
   `ID` int NOT NULL,  
   `PATH` char(20) NOT NULL DEFAULT '',  
   PRIMARY KEY (`ID`) 
   ) ENGINE=InnoDB DEFAULT CHARSET=utf8; 

   CREATE TABLE `FILE` (  
   `ID` int NOT NULL AUTO_INCREMENT,  
   `NAME` char(20) NOT NULL DEFAULT '',  
   `PATH` char(60) NOT NULL DEFAULT '',  
   `ATTRIBUTE` char(10) NOT NULL DEFAULT '',  
   `TIME` char(10) NOT NULL DEFAULT '',  
   `NOA` int NOT NULL DEFAULT 1,  
   `ISFOLDER` boolean NOT NULL DEFAULT false,  
   PRIMARY KEY (`id`) 
   ) ENGINE=InnoDB DEFAULT CHARSET=utf8; 

   CREATE TABLE `REQUEST` (  
   `ID` int NOT NULL AUTO_INCREMENT,  
   `TYPE` int NOT NULL DEFAULT 0,  
   `FRAGMENTID` int NOT NULL DEFAULT 0,  
   `DEVICEID` int NOT NULL DEFAULT 0,  
   PRIMARY KEY (`ID`) 
   ) ENGINE=InnoDB DEFAULT CHARSET=utf8; 

   CREATE TABLE `USER` (  
   `ID` int NOT NULL AUTO_INCREMENT,  
   `NAME` char(20) NOT NULL UNIQUE DEFAULT '', 
   `PASSWD` char(20) NOT NULL DEFAULT '',  
   PRIMARY KEY (`ID`) 
   ) ENGINE=InnoDB DEFAULT CHARSET=utf8;

   之后就会看到表单创建成功

   mysql> SHOW TABLES;
   +-----------------------+
   | Tables_in_dfs |
   +-----------------------+
   | device                |
   | file                  |
   | fragment              |
   | request               |
   | user                  |
   +-----------------------+
   5 rows in set (0.02 sec)

5. 加一个 id 为1， isonline 为1 的 device。

   INSERT INTO device (id, isonline)
       -> VALUES
       -> (1,1);

   mysql> SELECT * FROM device;
   +----+----+------+----------+------+
   | ID | IP | PORT | ISONLINE | RS   |
   +----+----+------+----------+------+
   |  1 |    |    0 |        1 |    0 |
   +----+----+------+----------+------+
   1 row in set (0.00 sec)

6. 修改 dataConnect 中的 ClientThread 中的 

   ClientThread{

   ​      client_socket: stream,

   ​      sentence: String::new(),

   ​      download_folder_path: PathBuf::from("xxxx/downloadFragment/"),

   ​      upload_folder_path: PathBuf::from("xxxx/uploadFragment/"),

   ​    }

   xxxx改成所需路径



# Introduction to Rust Web Applications

网址：https://erwabook.com/intro//

## Migration

- 提供 do 和 undo 两种 SQL 命令

- diesel migration generate task

  create a subdirectory under ./migrations/ which is named with a date/time stamp and the suffix "_task". In that timestamped subdirectory are two stub files: up and down

- up.sql:

  ```sql
  CREATE TABLE task (    
  	id INTEGER NOT NULL,    
  	title TEXT NOT NULL,    
  	PRIMARY KEY (id) 
  ); 
  ```

  down.sql

  ```sql
  DROP TABLE task;
  ```

- test: diesel migration run

- 查看表单：echo .dump | sqlite3 testdb.sqlite3

- redo = rollback + re-run

## Database Access Layer

### Inserting a Task

- diesel.toml

  by diesel setup

- src/lib.rs

  ```
  #[macro_use]
  extern crate diesel;
  
  pub mod db;
  //let Rust know we're making the db module
  ```

- module structure:

  ```
  mytodo
    +-- db
    |    +-- models
    |    +-- schema
    +-- rest
  ```

- removing the existing `schema.rs` file

  in diesel.toml

  [print_schema] 

  file = "src/db/schema.rs"

- src/db/models.rs:

  ```
  use super::schema::task;
  
  #[derive(Insertable)]
  #[table_name = "task"]
  pub struct NewTask<'a> {
      pub title: &'a str,
  }
  //define some types that we can use for reading //and writing the database
  ```

- src/db/mod.rs:

  automagically give us code to perform database inserts

  ```
  use diesel::{prelude::*, sqlite::SqliteConnection};
  
  pub mod models;
  pub mod schema;
  
  pub fn establish_connection() -> SqliteConnection {
      let db = "./testdb.sqlite3";
      SqliteConnection::establish(db)
          .unwrap_or_else(|_| panic!("Error connecting to {}", db))
  }
  
  pub fn create_task(connection: &SqliteConnection, title: &str) {
      let task = models::NewTask { title };
  
      diesel::insert_into(schema::task::table)
          .values(&task)
          .execute(connection)
          .expect("Error inserting new task");
  }
  ```

### Development Tool

- create a new binary

  $ mkdir src/bin/

- src/bin/todo.rs:

  ```
  use std::env;
  use mytodo::db::{create_task, establish_connection};
  
  fn help() {
      println!("subcommands:");
      println!("    new<title>: create a new task");
  }
  
  fn main() {
      let args: Vec<String> = env::args().collect();
  
      if args.len() < 2 {
          help();
          return;
      }
  
      let subcommand = &args[1];
      match subcommand.as_ref() {
          "new" => new_task(&args[2..]),
          _ => help(),
      }
  }
  ```

- run

  cargo run --bin todo new 'xxx'

  echo 'select * from task;' | sqlite3 testdb.sqlite3

  1|xxx

### Querying Tasks

- use a struct derived from `Queryable` to perform queries

  src/db/models.rs:

  ```
  #[derive(Queryable)]
  pub struct Task {
      pub id: i32,
      pub title: String,
  }
  ```

- want to let the database engine automatically assign the id instead of setting the id

  src/db/mod.rs:

  ```
  pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
      schema::task::table
          .load::<models::Task>(connection)
          .expect("Error loading tasks")
  }
  ```

  `src/bin/todo.rs`:

  ```
  fn show_tasks(args: &[String]) {
      if args.len() > 0 {
          println!("show: unexpected argument");
          help();
          return;
      }
  
      let conn = establish_connection();
      println!("TASKS\n-----");
      for task in query_task(&conn) {
          println!("{}", task.title);
      }
  }
  ```

- run

  $ cargo run --bin todo show

### Database Layer Wrap-Up

### Database Layer Exercises

- add 'done' column to table
- add delete task subcommand

## REST API Layer

- add Rocket to Cargo.toml

  ```
  [dependencies]
  diesel = { version = "1.0.0", features = ["sqlite"] }
  rocket = "0.4.2"
  ```

- backend binary

  ```
  #![feature(proc_macro_hygiene, decl_macro)]
  
  #[macro_use]
  extern crate rocket;
  
  use mytodo::db::models::Task;
  use mytodo::db::{query_task, establish_connection};
  
  #[get("/tasks")]
  fn tasks_get() -> String {
      "this is a response\n".into()
  }
  
  fn main() {
      rocket::ignite()
          .mount("/", routes![tasks_get])
          .launch();
  }
  ```

  cargo run --bin backend -> listening on localhost port 8000

- query database

  return dynamic data

  ```
  #[get("/tasks")]
  fn tasks_get() -> String {
      let mut response: Vec<String> = vec![];
  
      let conn = establish_connection();
      for task in query_task(&conn) {
          response.push(task.title);
      }
  
      response.join("\n")
  }
  ```

- json

  Cargo.toml:

  ```
  [package]
  name = "mytodo"
  version = "0.1.0"
  authors = ["Your Name <you@example.com>"]
  edition = "2018"
  
  [dependencies]
  diesel = { version = "1.0.0", features = ["sqlite"] }
  rocket = "0.4.2"
  serde = { version = "1.0", features = ["derive"] }
  
  [dependencies.rocket_contrib]
  version = "0.4.2"
  default-features = false
  features = ["json"]
  ```

  backend.rs:

  ```
  #[macro_use]
  extern crate rocket_contrib;
  #[macro_use]
  extern crate serde;
  
  #[derive(Serialize)]
  struct JsonApiResponse {
      data: Vec<Task>,
  }
  
  {
      "data": [
          { "id": 1, "title": "do the thing" },
          { "id": 2, "title": "get stuff done" },
      ]
  }
  ```

  fix bugs:

  both lib.rs and backend.rs: 

  \#[macro_use] extern crate serde;

  db/models.rs: 

  \#[derive(Queryable, Serialize)] pub struct Task {    pub id: i32,    pub title: String, }

  backend.rs:

  use rocket_contrib::json::Json;

  push the tasks we get back from `query_task` onto a response object, and then convert that to Json on the way out:

  ```
  #[get("/tasks")]
  fn tasks_get() -> Json<JsonApiResponse> {
      let mut response = JsonApiResponse { data: vec![], };
  
      let conn = establish_connection();
      for task in query_task(&conn) {
          response.data.push(task);
      }
  
      Json(response)
  }
  ```

## Browser-based frontend UI

seed

# Rocket



## Overview

- dynamic params <name>
- FromData: automatically attempt to parse the request body into the `Form` and call the function
- request guards: ApiKey
- responder: return type must implement responder
- launching: mount launch
- 3-steps: validation, processing, response

## QuickStart



## Requests

- validate dynamic path segment, *several* dynamic path segments, incoming body data, query strings, forms, and form values, incoming or outgoing format of a request,  arbitrary, user-defined security or validation policies.

- Rocket route attribute: get`, `put`, `post`, `delete`, `head`, `patch`, or `options

- dynamic path: #[get("/hello/<name>")]

- multiple segments: 

  ```
  #[get("/page/<path..>")]
  fn get_page(path: PathBuf) { /* ... */ }
  ```

- forwarding: When a parameter type mismatch occurs, Rocket *forwards* the request to the next matching route, if there is any. Routes are attempted in increasing *rank* order.

  ```
  #[get("/user/<id>")]
  fn user(id: usize) { /* ... */ }
  
  #[get("/user/<id>", rank = 2)]
  fn user_int(id: isize) { /* ... */ }
  
  #[get("/user/<id>", rank = 3)]
  fn user_str(id: &RawStr) { /* ... */ }
  ```

- default ranking: 

- query strings: #[get("/hello?wave&<name>")]

- Optional Parameters: use `Option<T>` as the parameter type

  ```
  #[get("/hello?wave&<name>")]
  fn hello(name: Option<String>) -> String {
      name.map(|name| format!("Hi, {}!", name))
          .unwrap_or_else(|| "Hello!".into())
  }
  ```

- request guards: Rocket将在调用处理程序之前自动调用请求保护程序的FromRequest实现。

- forwarding guards: 

  ```
  use rocket::response::{Flash, Redirect};
  
  #[get("/login")]
  fn login() -> Template { /* .. */ }
  
  #[get("/admin")]
  fn admin_panel(admin: AdminUser) -> &'static str {
      "Hello, administrator. This is the admin panel!"
  }
  
  #[get("/admin", rank = 2)]
  fn admin_panel_user(user: User) -> &'static str {
      "Sorry, you must be an administrator to access this page."
  }
  
  #[get("/admin", rank = 3)]
  fn admin_panel_redirect() -> Redirect {
      Redirect::to(uri!(login))
  }
  ```

- cookies: important, built-in request guard

  private cookies: Cookies::add()	

  secret key: 256-bit, 私有cookie只能使用加密时使用的同一密钥进行解密

  at most one `Cookies` instance be active at a time

- format

  ```
  #[post("/user", format = "application/json", data = "<user>")]
  fn new_user(user: User) { /* ... */ }
  ```

  当路由指示有效负载支持方法（PUT、POST、DELETE和PATCH）时，format route参数指示Rocket检查传入请求的Content-Type头。只有内容类型标头与格式参数匹配的请求才会与路由匹配。当路由指示非有效负载支持方法（GET、HEAD、OPTIONS）时，format route参数指示Rocket对照传入请求的Accept报头进行检查。路由只接受与媒体类型匹配的请求。

- body data:

  ```
  #[post("/", data = "<input>")]
  fn new(input: T) { /* .. */ }
  ```




# PPT

所有权

rust 中每一个值被有且仅有一个所有者持有，当所有者离开作用域时，这个值就会被丢弃。

1. Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
2. 值有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

- 当 `s` **进入作用域** 时，它就是有效的。
- 这一直持续到它 **离开作用域** 为止。



Rust 中的每一个引用都有其 **生命周期**（*lifetime*），也就是引用保持有效的作用域。

它们允许你使用值但不获取其所有权

生命周期

Rust 中的引用允许你使用值但不获取其所有权，但都有其生命周期，也就是引用保持有效的作用域。



rust 中每一个值被有且仅有一个所有者持有，当所有者离开作用域时，这个值就会被丢弃。在有 垃圾回收的语言中，  垃圾回收会自动记录并清除不再使用的内存。没有 垃圾回收的话，我们就要自己识别出不再使用的内存并调用代码显式释放。但 Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放，所以可以在没有垃圾回收器的情况下实现内存回收，不需要 close 等函数。



总的设计相关

what why how

现有系统比较



## PPT 思路

- 

- 项目 demo 展示

- 项目框架

  client和server：客户端运行，把文件放进指定文件夹，然后会被分块，传输给服务端，服务端则会把碎片分发给各个客户端，然后在数据库里做相应记录

  前后端：本来是打算采用Node.js来写，但是后续调研发现WebAssembly编译Rust存在较大的局限性，所以整个项目编译为WebAssembly是不现实的，主要还是在前端做一定的编译。然后随后调研，查阅资料后发现，考虑到17级已经实现了比较完整的前后端。那么我们初步设定目标就是写好client和server

  然后后续为了和我们的项目契合，以及统一项目工程，是考虑的逐步用rust取代原来的Tomcat服务器为中心构建的前后端。

  目前我们是考虑在前后端分离，采用actix_web框架实现后端，采用Seed实现前端。目前后端工作基本完成，还在调试阶段。主要考虑的是对采用Tomcat启动的前端的事件作ajax响应，把其发送到后端，然后后端和数据库做交互，最后把结果返回给前端

  前端这一块，考虑用seed框架来完成，其内置了把rust编译为webAssembly的几个包，是比较理想的完成方式，但是由于前端开始的相对较晚(前期准备不充分，低估了rust写前端的难度)，所以目前进度不算理想，可能考虑后续完善。

- 项目架构介绍
  - client
  - server
  - 后端 actix-web
  - 前端 seed
  
- 项目亮点介绍
  - 纠删码
  
    
  
  - rust
  
    - 所有权&生命周期
    - 错误处理准确且多样
      - panic：溢出等错误，直接报错，没有修复的可能
      - 可恢复的错误：Result 和 Option 类型，出错时返回错误，可以使用 unwrap 返回 panic，或者使用 expect 返回指定错误信息
    - Java的整型操作均是封装好的（并没有溢出检查），而 Rust 会在调试模式时进行溢出检查，在发布模式下进行封装时不做检查。
  
  - 锁机制
  
    先是创建锁（其包含一个状态值），并分别 clone 到 folderScanner 和ServerConnecter 两个线程；然后主线程调用 wait 进入睡眠；最后若另外两个线程发生错误，则会获取锁并修改状态值，再唤醒主线程；主线程检测到状态值改变，则会输出相应报错信息，最后终止所有 client 的线程
  
  - web 服务器搭建
  
    采用 actix-web 框架做后端以实现对路由的挂载，以及与数据库的交互
  
    预期采用 seed 框架做前端以实现对浏览器页面的加载与交互，并利用 rust 的 web-pack 等包编译 rust 为 WebAssembly 以运行在浏览器中：已在设计中，还未完成
  
  - 
  
- 与现有系统比较

  - MooseFS	用perl编写，比较轻量级，国内用的人比较多，易用，稳定，对小文件很高效，性能相对较差，对master服务器有单点依赖。
  - MogileFS    主要用在web领域处理海量小图片，效率相比mooseFS高很多，只适合存储静态只读小文件。perl编写的代码， 对外提供API来进行使用， 因为需要安装很多依赖的第三方perl包，搭建相对比较复杂一点。
  - ceph    彻底的分布式，没有单点依赖，用C编写，性能较好。基于不成熟的btrfs，其本身也非常不成熟 
  - 目前网络上比较流行的商用分布式文件系统如GFS，AFS过于面向大型企业与团体，性能要求高、可靠性要强，只支持在局域网上运行；

- 意义价值、愿景
- Q&A





介绍

分布式文件系统可以将位于多个地点多个系统的文件，通过网络进行节点间的通信和数据传输，组成一个文件系统网络，同时用户只需要像使用本地文件系统一样管理和存储文件系统中的数据。

Rust 语言的设计理念是，凡是能够引发程序内存出错的操作，都是不安全不能通过编译的，这样就使得 Rust 在零开销的基础上，保证了系统安全可靠。同时，Rust 作为一种底层语言，也可以带来高性能和低资源利用等优良特性。

我们的项目目的是创建一个小型便捷的家庭分布式文件系统，选取了以基于 Java 的17级小型分布式文件系统为参考，完全使用 Rust 语言完成了 client、server 以及 web 后端，并与17级项目的 web 前端做了成功衔接，达到了我们的预期目标。



数据库

首先，我们基于廖雪峰的 SQL 基础教程学习了数据库的基本知识，学习了有关关系模型、查询数据 SELECT 语句、修改数据 UPDATE 语句、删除数据 DELETE 语句、增加数据 INSERT 语句和具体使用 MySQL 的方法等等。而后我们搜索了 Rust 中的 MySQL 使用，找到了[Rust 文档中的 MySQL 官方库](https://docs.rs/mysql/17.0.0/mysql/)，在学习了文档中的示例之后，我们首先使用示例进行了测试，而后仿照示例完成了我们的改写代码，在实际测试中也完善了错误处理，最后成功实现了数据库模块的功能。





