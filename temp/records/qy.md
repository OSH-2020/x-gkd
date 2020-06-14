* 一些调用其他模块中内容的变量，可能涉及到漏设置 mut 属性！！

### ServerConnecter.rs

目前为1.0版。这个文件定义了 ServerConnect 结构体，以及 new, init, run, stopConnect 四个方法。这个文件可以通过编译，但还有一些 warning 没有解决。

* 关于 crate 结构如何组织的问题还没有解决。目前先按照原来项目的目录存放文件。有新的方案之后可能会需要在代码中增加几行。

* 对应原文件的类 syn 字段在这里没有实现。目前我不了解 syn 在这里的作用和必要性。查找到 Rust 有 `Trait std::marker::Sync` 。

  https://moshg.github.io/rust-std-ja/std/marker/trait.Sync.html

* 还没有写部分函数的错误处理。目前多用 unwrap 或 expect 进行简单的错误处理，一出错就会调用 panic 关闭程序，还需要进一步修改区分一些可恢复法错误。

* 使用 Bufreader 对 TcpStream 进行按行读的操作。为了同时使用 BufReader 读和使用原有的 TcpStream 写，在创建 BufReader 对象前调用了 socket.try_clone() ，对于这个函数的一些细节还不是很清楚。 

* 没有查到原 ServerConnecter.java 中的 client.Client.getRS() 的作用及其对应方法，原文件第 68 行的调用没有对应实现。

* 有涉及其他文件中定义的结构体的情况，方法接口可能不一致。目前写为：

  ~~~rust
  let f_manager = FragmentManager::new(request_id, fragment_id, ftype);
  f_manager.submit();
  ~~~

  注：type 是 Rust 关键字，因此此处变量命名为 ftype 。



### FileTransporter.rs

这个文件定义了 recv_file 和 send_file 两个函数。

recv_file 调用时需要在 TcpStream 的文件内容之前有一个 big endian 的 64 位文件大小参数。

* 错误处理待完善
* 根据调用情况， DataInputStream DataOutputStream 对应都视为 TcpStream
* 将参数类型改为了 &TcpStream，否则如果使用 TcpStream，函数调用完毕返回时不能返还所有权。
* 目前接收文件是1024字节一次，分多次接收。发送文件是一次性发送，无法发送 4096 字节以上的文件。
* 还没有找到得到文件大小的方法，即 f.length() 的对应实现。
* 涉及到 client\src\fileDetector\FileUploader, client\src\connect\FragmentManager 中的调用，对应函数传参还没有更改。



### dataConnect/ServerThread.rs

这个文件定义了 ServerThread 结构体，以及构造函数和 run 函数，有调用 ClientThread 文件中的内容。

* 原文件中的 ServerThread 类是从 Thread 类继承而来的，但 Rust 没有类继承，可能因此出问题。如果别的地方用到其他继承来的方法或字段，再对本文件进行相应补充。



### dataConnect/ClientThread.rs

* 调用 FileTransporter.rs 中的函数，目前对于流式输入输出的实现还有待修改。
* 调用 query.rs 和 DeviceItem.rs 中内容，还不知道具体组织形式和调用方法，因此本文档这部分内容未通过编译。这一问题在 controlConnect/ClientThread.rs 中也存在。



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

```
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

```
use std::io::prelude::*;
use std::net::TcpStream;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();

    let t = stream.read(&mut [0; 128]).unwrap(); 
    println!("read bytes: {}", t);
} 
```

运行客户端程序的命令行窗口可以输出读到的字节数。

测试 recv_file 函数，其中 18 为手动计算的文件大小。

```
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

fn main(){
    let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
    //let mut res = String::new();
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

