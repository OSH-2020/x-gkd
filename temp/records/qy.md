### ServerConnecter.rs

目前为1.0版。这个文件定义了 ServerConnect 结构体，以及 new, init, run, stopConnect 四个方法。这个文件可以通过编译，但没有解决大部分的 warning。

run 方法中，while self.connecting 循环最后为线程 sleep 3秒，每次循环开始都尝试连接 server ，并输出连接是否成功的信息。

* 关于 crate 结构如何组织的问题还没有解决。目前先按照原来项目的目录存放文件。有新的方案之后可能会需要在代码中增加几行。

* 对应原文件的类 syn 字段在这里没有实现。目前我不了解 syn 在这里的作用和必要性。查找到 Rust 有 `Trait std::marker::Sync` 。

  https://moshg.github.io/rust-std-ja/std/marker/trait.Sync.html

* 还没有写部分函数的错误处理。目前多用 unwrap 或 expect 进行简单的错误处理，一出错就会调用 panic 关闭程序，还需要进一步修改区分一些可恢复法错误。

* 使用 Bufreader 对 TcpStream 进行按行读的操作。为了同时使用 BufReader 读和使用原有的 TcpStream 写，在创建 BufReader 对象前调用了 socket.try_clone() ，对于这个函数的一些细节还不是很清楚。 




### FileTransporter.rs

这个文件定义了 recv_file 和 send_file 两个函数。

recv_file 调用时需要在 TcpStream 的文件内容之前有一个 big endian 的 64 位文件大小参数。

* 错误处理待完善

* 根据调用情况， DataInputStream DataOutputStream 对应都视为 TcpStream

* 将参数类型改为了 &TcpStream，否则如果使用 TcpStream，函数调用完毕返回时不能返还所有权。

* 发送和接收文件都是1024字节一次，分多次 read 或 write 完成。目前没有实现如何写入确定字节数的内容。

* 调用 file::read() 时使用 unwrap() 处理错误

  `This function does not provide any guarantees about whether it blocks waiting for data, but if an object needs to block for a read and cannot, it will typically signal this via an [Err] return value.`



### dataConnect/ServerThread.rs

这个文件定义了 ServerThread 结构体，以及构造函数和 run 函数，有调用 ClientThread 文件中的内容。

* 原文件中的 ServerThread 类是从 Thread 类继承而来的，但 Rust 没有类继承，可能因此出问题。如果别的地方用到其他继承来的方法或字段，再对本文件进行相应补充。



### dataConnect/ClientThread.rs

run 方法接收一行命令，然后根据命令类型 1-6 ，调用对应的其他方法进行处理。

本结构体的方法只接收一次命令，没有类似 while status 的循环。

* 对原文件中结构体的修改：删去 inFromClient 和 outToClient 两个字段，通过 Client_socket: TcpStream 直接完成这两个字段的功能。由于 Rust Vec\<String\> 使用不便，将字符串数组 String[] command 改为 sentence: String，使用时需要 let command:Vec<&str> = self.sentence[..].split(' ').collect(); 。
* query 结构体如何新建还未解决，query.closeConnection() 方法似乎还未实现

### controlConnect/ClientThread.rs

* ```
          //以下两行未实现：（推测为心跳机制保持连接功能）
          //clientsocket.setKeepAlive(true);
          //clientsocket.setSoTimeout(60000);
  ```



## 代码测试

### FileTransporter.rs

目前只是简单测试基本功能，没有对网络延迟、文件过大等可能情况测试。采用了将待测试内容复制到 main.rs 中的方法。如果调用与 main.rs 在同一目录下的文件，只要 `pub mod 文件名;` 即可引入。调用时以 `文件名::函数()` 格式调用。

测试文件服务端（提供 TCPListener，监听接收连接，提供连接线程执行的函数，调用待测试的函数）

其中 test1.txt 是在 crate 根目录下的文件。

```rust
use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("test1");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client1(stream)	// or handle_client2(stream)
                });
            }
            Err(e) => { return }
        }
    }
    println!("test end!");
}

fn handle_client1(stream: TcpStream) {
    let mut f1 = File::open("test1.txt").unwrap();
    println!("test2");
    send_file(f1, stream);
}
```

测试文件客户端（与对应地址连接）

我没有找到 windows cmd 上与 nc 对应的命令，查到有 netsh ，但使用看起来很复杂，于是没有再仔细学习，采用了再运行一个程序的方式来连接。

测试 send_file 函数：

```rust
use std::io::prelude::*;
use std::net::TcpStream;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    let t = stream.read(&mut [0; 128]).unwrap(); 
    println!("read bytes: {}", t);
} 
```

运行客户端程序的命令行窗口可以输出读到的字节数，为末尾用 \0 填充的 1024 字节。（因为目前不知道如何用 write 写入确定的字节数）

测试 recv_file 函数，其中 18 为手动计算的文件大小。

```rust
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    let i: i64 = 18;
    let t1 = stream.write(&i.to_be_bytes()).unwrap(); 
    let t2 = stream.write(b"send to test2.txt\n").unwrap(); 
    stream.flush();
    println!("write bytes: {}", t1 + t2);
    std::thread::sleep_ms(2000);
}  
```

运行后打开 test2.txt 可以看到 "send to test2.txt\n" 的内容。



测试 send_file 函数时，为了尝试输出读到的内容，我将 read 函数改为 read_to_string ，并输出相应 String 变量，输出变为：

```
read bytes: 4104
read content: "\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{12}this is test1.txt\n\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}...
```

猜测 4104 字节是 read_to_string 方法读数据量的上限。目前还不清楚 read 方法和 read_to_string 方法实现的根本区别。

使用 Ctrl + C 退出服务端程序时会报 error：（个人觉得这应该不算一个错误）

```
error: process didn't exit successfully: `target\debug\OSHtest.exe` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
```

### ServerConnect.rs

目前可以跑，但 run 方法从 readline 函数调用之后的循环内的内容似乎都没能执行，并且从那个地方直接跳入下一次循环（根据测试，是直接将函数从头开始执行）。thread::sleep 函数从未执行，移动到 read_line 之前也不能成功执行，因此终端上会不停输出 "Connect to server successfully(control)!"

read_line 的问题也可能是因为我对 TCP 通信方式理解有问题，测试文件不合理。

测试文件服务端：（main 函数部分与 FileTransporter.rs 测试文件相同）

```rust
fn handle_client(stream: TcpStream) {
    let mut cserver = ServerConnect::ServerConnecter::new(1);
    let s = String::from("127.0.0.1");
    let i:u16 = 8000;
    cserver.init(s, i);
    cserver.run();
}
```

测试文件客户端：

```rust
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    let t1 = stream.write(b"0 0 1\n").unwrap(); 
    let t2 = stream.write(b"0 0 1\n").unwrap(); 
    stream.flush();
    println!("write bytes: {}", t1 + t2);
    std::thread::sleep_ms(2000);
} 
```



## Seed 相关

### Rust Web Applications

* ULID：一个库，用于生成独特 ID 。基于时间戳和随机数生成 ID ，它们可根据时间排序。不同机器上的 ULID 之间可能冲突。
* ORM ：对象关系映射
* REST API：REST 为 Representation State Transfer ，表现层状态转化。决定网络资源呈现形式。[REST API 介绍](https://www.jianshu.com/p/75389ea9a90b)
* URI ：统一资源定位符。只代表资源实体，与表现形式无关。



### Seed-rs-realworld

lib.rs 中的 Model 为枚举体，是由于一整个 app 中可能出现 home, login, article 等不同的页面，每个页面在对应的 rs 文件中都定义了自己的 Model 结构体。同一时刻同一线程内不会有多个页面存在，lib.rs 即为这些 Model 的枚举。

#### /src/session.rs

* session    [Session 简介](https://blog.csdn.net/weixin_42217767/article/details/92760353)

  记录一系列状态。Session 与 cookie 功能效果相同。区别在于 Session 是记录在服务端的，而 Cookie 是记录在客户端的。

  realworld 中的 session 结构体记录当前是游客状态还是已登录。

* Token    [Token 简介](https://www.jianshu.com/p/24825a2683e6)

  Token是服务端生成的一串字符串，以作客户端进行请求的一个令牌，当第一次登录后，服务器生成一个Token便将此Token返回给客户端，以后客户端只需带上这个Token前来请求数据即可，无需再次带上用户名和密码。

  Token 的应用是为了减少查询数据库频次。

  realworld 中 session Login 字段使用到 src/entity/viewer::Viewer ，该结构体中有名为 auth_token 的 String 字段，应该为上述的生成 Token。

#### /src/page/register.rs

```rust
///src/page/register.rs 中 Model：
pub struct Model {
    session: Session,
    problems: Vec<Problem>,
    form: Form,
}
///src/entity/form.rs:
pub struct Form<T: FormField>(IndexMap<FieldKey, T>);
pub enum Problem {
    InvalidField {
        field_key: &'static str,
        message: Cow<'static, str>,
    },
    ServerError {
        message: Cow<'static, str>,
    },
}
///src/entity/form/register.rs
pub type Form = form::Form<Field>;
pub enum Field {
    Username(String),
    Email(String),
    Password(String),
}
```

其中 Field 三个字段与网页注册需要输入的相同，但 Field 为枚举体。

目前还不知道 register.rs 的功能应该如何概括，register 页面是哪一个页面、用户是否能进入这个页面，但这个模块肯定是登录注册模块需要的。

* [indexmap库](https://github.com/bluss/indexmap)

  一个动态的 hash 表，可以在一定范围内保持插入顺序。

#### /src/page/login.rs

login.rs 文件的大部分内容（包括 Model 结构体，init 函数等）都与 register.rs 文件相同。根据 viewer 函数，register 为 Sign up 注册页面，login 为 Sign in 登录页面。

两文件 Msg 枚举体不同。register.rs 中 RegisterCompleted 字段名改为 LoginCompleted ，但变量类型还是一样的。

两者 update 函数对于 Msg::FormSubmitted 的处理不同。但 RegisterCompleted 和 LoginCompleted 的处理完全相同。

#### /src/page/profile.rs

```rust
pub struct Model<'a> {
    session: Session,
    errors: Vec<ErrorMessage>,
    selected_feed: SelectedFeed,
    feed_page: PageNumber,
    author: Status<'a, Author>,
    feed: Status<'a, article::feed::Model>,
}
pub enum SelectedFeed {
    MyArticles,
    FavoritedArticles,
}
enum Status<'a, T> {
    Loading(Username<'a>),
    LoadingSlowly(Username<'a>),
    Loaded(T),
    Failed(Username<'a>),
}
```

其中 PageNumber 为 usize，在 /entity/page_number.rs 中定义。

Model 中只有 session, errors 字段是我们需要的，其他都跟 article 有关。本文件可能需要大幅重写。

* feed    [什么是 feed 流](https://www.zhihu.com/question/20690652)

  feed 指满足用户需求的信息单元。

  可以确定关于 "feed" 的代码都是与 realworld 中 article 有关，是我们不需要的。

#### /src/page/home.rs

```rust
pub struct Model<'a> {
    session: Session,
    selected_feed: SelectedFeed,
    feed_page: PageNumber,
    tags: Status<Vec<Tag>>,
    feed: Status<article::feed::Model>,
}
```

Model 中只有 session 与我们需要的有关。

#### /src/storage

使用 seed::storage, serde_json 库。存储 /entity/viewer::Viewer 实体的信息。

```rust
///entity/viewer::Viewer
pub struct Viewer {
    pub profile: Profile,
    pub auth_token: String,
}
```

