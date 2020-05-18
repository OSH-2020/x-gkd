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

这个文件定义了 recv_file 和 send_file 两个函数。recv_file函数还没有通过测试。

* 这两个收发文件的函数是可以测试的，但我总是不能成功运行 read 函数，报错`thread 'main' panicked at 'Problem read file: Os { code: 5, kind: PermissionDenied, message: "拒绝访问。" }', src\main.rs:19:23`
* 错误处理待完善
* 我没有找到和 DataInputStream DataOutputStream 对应的处理方式，目前使用输出到 stdout ，输入到 stdin 的方式实现。
* 目前接收文件是1024字节一次，分多次接收。发送文件是一次性发送。
* 还没有找到发送文件时指定写入字节数的方法，有待更改。



### ServerThread.rs

这个文件定义了 ServerThread 结构体，以及构造函数和 run 函数，有调用 ClientThread 文件中的内容。

* 原文件中的 ServerThread 类是从 Thread 类继承而来的，但 Rust 没有类继承，可能因此出问题。如果别的地方用到其他继承来的方法或字段，再对本文件进行相应补充。