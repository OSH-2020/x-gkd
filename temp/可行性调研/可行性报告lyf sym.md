# 1 实现方案与可行性分析

## 1.1客户端服务程序

本项目使用 Rust 改写17级“基于互联网网页的小型分布式文件系统”项目，用 Rust 改写原项目的 Java 客户端。与原项目相同，本项目的客户端也采用与tcp 协议的 socket 通信与服务器建立连接。

客户端利用 Config 读取配置文件，为Rust应用程序组织分层或分层配置。Config可设置一组默认参数，然后可以通过合并各种来源的配置来扩展它们。

客户端提供以下几个功能：

- 获取本地目录

  原项目的 Java 客户端使用 `java.io.File` 类获取文件列表，本项目选用 Rust 改写，使用`std::io::fs`模块处理文件系统，列出本地机器参与共享的文件列表。

```rust
//std::io::fs 模块包含几个处理文件系统的函数
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;
```

```Rust
    match fs::read_dir("a") {
    //在主函数中读取目录的内容，返回 `io::Result<Vec<Path>>`
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }
```

- 对文件进行分块

  客户端使用erasure code 算法，对服务器发来的文件进行分块。此算法可依据 github 上的开源项目 [reed-solomon-erasure](https://github.com/darrenldl/reed-solomon-erasure) 实现。

- 向服务器发送文件碎片、接收服务器传来的碎片

  - 使用到的标准库：

    [`std::net:TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html) 用于监听连接

     [`std::net::TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html) 用于传输数据。

  - 过程：在 `TcpListener` `  accept` 连接或 `connect` 到一个远程主机后，将在本地和远程套接字间创建一个`TcpStream` ，数据（文件碎片）通过写入或读取它以进行传输。

  - 示例：

  ```Rust
  use std::io::prelude::*;
  use std::net::TcpStream;
  
  fn main() -> std::io::Result<()> {
      let mut stream = TcpStream::connect("127.0.0.1:34254")?;
  
      stream.write(&[1])?;
      stream.read(&mut [0; 128])?;
      Ok(())
  } // the stream is closed here
  ```


- 响应服务器删除本地文件

  接收到服务器删除文件的请求后，依据文件路径，使用 `std::fs::remove_file` 删除文件，示例代码如下

```
use std::fs;
fn main() {
   fs::remove_file("data.txt").expect("could not remove file");
   println!("file is removed");
}
```

## 1.2 服务器

### 1.2.1 服务器功能：

​		基于互联网网页的小型分布式文件系统主要实现了如下功能：

- 连接类：接收、回复、转发服务请求与控制信息；收发数据（文件碎片）。
- 数据管理类：维护云共享文件索引；维护各个客户端的状态信息；记录、处理当前等待响应的文件请求。

### 1.2.2 Java 源码实现方法：

​		首先要介绍套接字（Socket）的含义：套接字是一种在应用程序与TCP／IP协议交互时，用来区分不同应用程序进程间的网络通信和连接的接口。主要有三个参数：通信的目的IP地址、使用的传输层协议(TCP或UDP)和使用的端口号。Socket可以看成在两个程序进行通讯连接中的一个端点，一个程序将一段信息写入Socket中，该Socket将这段信息发送给另外一个Socket中，使这段信息能传送到其他程序中。如下图：

​		![image-20200325213315824](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200325213315824.png)

​		Host A上的程序A将一段信息写入Socket中，Socket的内容被Host A的网络管理软件访问，并将这段信息通过Host A的网络接口卡发送到Host B，Host B的网络接口卡接收到这段信息后，传送给Host B的网络管理软件，网络管理软件将这段信息保存在Host B的Socket中，然后程序B才能在Socket中阅读这段信息。要通过互联网进行通信，至少需要一对套接字，一个运行于客户机端，称之为ClientSocket，另一个运行于服务器端，称之为serverSocket。

![image-20200325234837370](C:\Users\sym\AppData\Roaming\Typora\typora-user-images\image-20200325234837370.png)

- ServerSocket 类在服务器端创建欢迎套接字。
- Socket 类在客户端或服务器端创建链接套接字。
- 用一系列在 MySQL 数据库中的 table 保存云共享文件索引，每台共享了文件的电脑均在数据库中对应两个 table，其一记录了这台电脑上共享的每个文件的唯一标识符和其逻辑位置；其二记录了这台电脑上各个文件的碎片的物理位置和其唯一标识符。
- 用一个在 MySQL 数据库中的 table 保存客户端，其中有：唯一标识符，在线情况，剩余空间及当前复杂维持与这个客户端的控制链接（TCP 链接）的线程的编号。
- 再用一个 table 记录当前网页提出的文件请求。当服务器收到来自客户端的心跳连接时，将查询文件请求表，如果发现有对客户端上文件的请求，则在回复心跳连接时将文件请求发给客户端并令其（通过服务器）将文件发往请求方，这样就能解决服务器不时时连接客户端的问题。

### 1.2.3 源码结构


- server

  - controlConnect

    - ClientThread  
    - ServerThread 

  - database

    - Deviceltem   //各种信息参数查询设置

    - FileItem   //各种信息参数查询设置

    - Query    closeConnection    queryFile//通过名字地址或ID或地址查找文件    

      ​               queryFragment//fragment, device, request, password, id

      ​               或者对上述对象 add, delete, alter

    - RequestItem

  - dataConnect

    - ClientThread	//send delete receive fragment              confirm//确定在线主机，碎片数量，判断如何发送
    - FileTransporter    //receive send files
    - ServerThread

  - DFS_server

### 1.2.4 Rust 实现

- std :: net
  TCP / UDP通信的网络原语。该模块提供了传输控制和用户数据报协议的网络功能，以及IP和套接字地址的类型。

  > TcpListener并TcpStream提供用于通过TCP进行通信的功能
  > UdpSocket 提供通过UDP进行通信的功能
  > IpAddr表示IPv4或IPv6的IP地址；Ipv4Addr和 Ipv6Addr分别是IPv4和IPv6地址
  > SocketAddr表示IPv4或IPv6的套接字地址；SocketAddrV4 和SocketAddrV6分别是IPv4和IPv6套接字地址
  > ToSocketAddrs与网络对象，如交互时使用的通用地址解析服务的特质TcpListener，TcpStream或UdpSocket
  > 其他类型是此模块中各种方法的返回值或参数类型

- std::thread

  正在执行的Rust程序由一组本机OS线程组成，每个本机线程都有自己的堆栈和本地状态。可以命名线程，并为低级同步提供一些内置支持。

  线程之间的通信可以通过 通道，Rust的消息传递类型以及其他形式的线程同步和共享内存数据结构来完成。

  当Rust程序的主线程终止时，即使其他线程仍在运行，整个程序也会关闭。但是，此模块提供了方便的功能，可以自动等待子线程的终止。

  可以使用thread::spawn函数产生一个新线程：

  ```rust
  use std::thread;
  
  thread::spawn(move || {
      // some work here
  });
  ```

  	在此示例中，生成的线程与当前线程“分离”。这意味着它可以超过其父级（产生它的线程），除非该父级是主线程。
		
  	父线程也可以等待子线程的完成。调用spawn产生JoinHandle，提供了join等待的方法：(该join方法返回一个thread::Result包含Ok子线程产生的最终值的内容，或者返回给子线程恐慌时Err调用的值panic!)

  ```
  use std::thread;
  
  let child = thread::spawn(move || {
      // some work here
  });
  // some work here
  let res = child.join();
  ```

  	该模块还为Rust程序提供了线程本地存储的实现。线程本地存储是一种将数据存储到全局变量中的方法，程序中的每个线程都有其自己的副本。线程不共享此数据，因此不需要同步访问。
		
  	线程能够具有关联的名称以用于识别。默认情况下，生成的线程是未命名的。要为线程指定名称，请使用构建线程，Builder并将所需的线程名称传递给Builder::name。要从线程内部检索线程名称，请使用Thread::name.

- rustc::traits::query

  特征查询界面的实验类型。该模块中定义的方法全部基于规范化，该规范化通过替换未绑定的推理变量和区域进行规范查询，从而可以更广泛地重用结果。可以在中找到此处定义的查询的提供程序 librustc_traits

- crate mysql

  提供了：

  - 完全用 Rust 写的MySql数据库驱动程序
  - 连接池

  特征：

  - macOS，Windows和Linux支持
  - MySql文本协议支持，即简单文本查询和文本结果集的支持；
  - MySql二进制协议支持，即支持预备语句和二进制结果集；
  - 支持大于2 ^ 24的MySql数据包；
  - 支持Unix套接字和Windows命名管道；

  安装

  ```rust
  [dependencies]
  mysql = "*"
  ```

# 2 理论依据与技术依据

## 2.1 Rust 改写

- 纠删码（Erasure Code）: 纠删码是一种前向错误纠正技术，用于在网络传输中避免包的丢失，以提高存储可靠性。它可将n份原始数据，增加m份数据，并能通过 n+m 份中的任意 n 份数据还原为原始数据。即如果有任意小于等于 m 份数据失效，仍然能通过剩下的数据还原出来。此为原17级项目“基于互联网的小型分布式文件系统”的一大特点。本项目将继承该项优势，用 Rust 实现纠删码。

- 网络数据传输：本项目所做的移动式文件访问的分布式文件系统，旨在使得分布在不同地点的各种设备可以共同维护，因此，采用使用TCP/IP协议的因特网进行数据交换。Rust 中有`std::net:TcpListener` `std::net::TcpStream` 等标准库可以调用。

## 2.2 Rust 调用 Java

​		使用j4rs项目在 Rust 中调用 Java。其主要思想是实现一个 crate，让用户轻松调用 Java，这样他们就可以从庞大的 Java 生态系统中受益。

- 注意 JNI（Java Native Interface） 所以需要的配置（例如 jvm 包含/链接本地共享库）。
- 创建一个直观、简单的 API 进行 Java 调用（Rust -> Java 方向）。
- 允许 Java -> Rust 回调。
- 无缝在在 Linux 或者 Windows 上使用 crate（当然，前提是安装了 Java）。
- 遵循 “Rust-first” 方式: Rust 代码可以创建和管理 JVM，而不是反过来

​		开始时候仅需要在 Cargo.toml 中定义 `j4rs` ： 

```text
[dependencies]
j4rs = "0.6"
```


​		使用 `j4rs`，下载 Maven 构件，以便在 Rust 应用程序中使用它调用其他 Java 库。

# 3 创新点

## 3.1 Rust 的优势

- 低资源占用

  控制资源使用，将内存与 CPU 占用降到最低。大多数运行条件下，一个Rust 程序比 Java 消耗的内存会少上一个数量级。

- 安全可靠

  Rust 的强大类型检查可防止多种级别的 Bug，确保开发者可随时明确状态是共享还是可变。在部署之前通过捕捉故障点来获得帮助。

- 生命周期及所有权规则

  虽然 Java 为使 GC 系统可管理，采用不分配内存的方式努力完善了内存回收机制，但有时却会导致代码过于复杂。Rust的生命周期及所有权规则，使得其可在没有GC（垃圾回收器）的情况下获取对象，使程序更少的出错。
  
- 优秀的鲁棒性

  Rust 在调试模式下的溢出检查，使得开发人员在测试期间能够发现更多问题，而发布模式下进行封装时不作检查，也提高了发布版本的执行效率。而Java的整型操作没有溢出检查。Rust 各种高要求的检查，与默认时的引用不变性，造就了 Rust 出色的鲁棒性。

- 错误处理

  任何一个线程发生 “panics” 时，都会被 Rust 认为是 RuntimeExceptions，Rust 会立即终止线程。并且 Rust 返回的错误信息比 Java 更具体，可以帮助程序员更好的理解错误点，完善代码。





# 参考文献

[erasure code 的 Rust 实现](https://github.com/darrenldl/reed-solomon-erasure)

标准库：

- [`std::net:TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html) 
- [`std::net::TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html) 
- [`std::fs`](https://doc.rust-lang.org/std/fs/index.html)
- [`std::net`](https://doc.rust-lang.org/std/net/index.html)

- [`std::net`](https://doc.rust-lang.org/std/net/index.html)
- [`std::traits::query`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc/traits/query/index.html)

[套接字（Socket）简介](https://blog.csdn.net/wangluqinglxq/article/details/38402759)

[Rust 调用 Java 方法](https://zhuanlan.zhihu.com/p/69412984)  [j4rs](https://github.com/astonbitecode/j4rs)