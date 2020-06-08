进度：client FileUploader还有一些bug

​			server Query



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

查询

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
添加

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
    // Also we assume that no e
    rror happened in `prepare`.
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

使用 use FileItem 解决，同时，假如使用 pub mod 就可以让 mod 有传递性。

先建立一个 Query 文件夹，在里面放 FileItem.rs