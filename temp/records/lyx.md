### 项目

出发点: 应用WebAssembly和Rust 实现浏览器上运行程序

找到了这样一个可行的项目，即分布式文件系统，那么我们改写出来的系统的特点在什么地方?

* 全打包为Wasm, 兼容性、移植性
* 与js交互(web服务应用)



### Rust总结:

* collect方法：把迭代器元素以集合形式返回

* 多线程

  * thread::spawn+闭包：创建新线程及其运行代码

    thread::sleep(Duration::from_millis(1))：让线程停止运行一段时间

    JoinHandle的join方法来让主线程等待线程运行结束

    move:移动数据到新线程（通过在闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权）

  ```rust
  let handle = thread::spawn(move || { 
      println!("Here's a vector: {:?}", v); 
  });
  ```

  * 通道 ，send，recv（阻塞直到接收到数据），try_recv(立即返回)： 三个函数均返回 Result<T, E>；。 send 函数获取其参数的所有权并移动这个值归接收者所有

    注意发送和接收端有一端关闭，则三个方法均会返回错误

    let (tx, rx) = mpsc::channel(); 多生产者单消费者（即可以有多个发送端(tx)，但只能有一个接收端(rx)）

    创建多个生产者:let tx1 = mpsc::Sender::clone(&tx); 

  * 共享状态并发

    互斥器(Mutex)：

    ```rust
    let handle = thread::spawn(move || { 
    	let mut num = counter.lock().unwrap(); 
        *num += 1; 
    });//但这样会把所有权移到该线程而导致其他线程无法使用
    ```

    改进：考虑多所有权：Mutex<T> 封装进 Rc<T> 中并在将所有权 

    移入线程之前克隆 Rc<T> :但这不安全，所以无法通过编译；所以考虑修改为原子引用计数Arc<T>

    ```rust
    let counter = Rc::new(Mutex::new(0));
    
    let counter = Rc::clone(&counter); 
    let handle = thread::spawn(move || { let mut num = counter.lock().unwrap(); *num += 1; });
    
    let counter = Arc::new(Mutex::new(0));
    let counter = Arc::clone(&counter); 
    let handle = thread::spawn(move || { let mut num = counter.lock().unwrap(); *num += 1; });
    ```

    

    std::sync::Condvar::wait和std::sync::Condvar::notify_one，std::sync::Condvar::notify_all

    原子类型std::sync::atomic

  * Send 标记 trait 表明类型的所有权可以在线程间传递,几乎所有的 Rust 类型都是 Send 的， 任何完全由 Send 的类型组成的类型也会自动被标记为 Send

    Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用，对于任意类型 T ，如果 &T （ T 的引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程，同理，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的

* unwrap():调用 `option.unwrap()` 来获取 `option` 中包裹的值,出错则直接panic

  如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) 

  ```
  use std::fs::File;
  
  fn main() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
  }
  ```

  这段程序相当于在 Result 为 Err 时调用 panic! 宏。两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息。

* 线程安全智能指针：为了在多个线程间共享所有权并允许线程修改 

  其值，需要使用 Arc<Mutex<T>> 。 Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次 只有一个 worker 能从接收端得到任务

  let receiver = Arc::new(Mutex::new(receiver)); 

* ```rust
  let x = Some(5);
  //以下代码等价
  match x {
      Some(_)  => {do_something();},
      None      => {}
  }
  
  if let Some(_) = x {
     do_something()
  }
  
  while let pattern = expression {
      do_something()
  }
  ```

* ```rust
  //Type别名
  type Name = String;
  let x: Name = "Hello".to_string();
  ```

* ```rust
  impl MyType{//两两等价
      fn doit(self){}
      fn doit(self: Self){}
  
      fn doit(&self) {}
      fn doit(self: &Self){}
  
      fn doit(&mut self) {}
      fn doit(self: &mut Self) {}
  }
  ```



### Yew

web worker

机制: Component：create，update，view三个函数

组件是 Yew 的基石。它们管理自己的状态，并可以渲染为 DOM。组件是通过实现描述组件生命周期的 `Component` trait 来创建的。

create:当一个组件被创建时，它会从其父组件以及一个 `ComponentLink` 接收属性（properties）。属性（properties）可用于初始化组件的状态，“link”可用于注册回调或向组件发送消息。

view:组件在 `view()` 方法中声明它的布局。Yew 提供了 `html!` 宏来声明 HTML 和 SVG 节点和它们的监听器及其子组件。这个宏的行为很像 React 中的 JSX，但是使用的是 Rust 表达式而不是 JavaScript

update:组件是动态的，可以注册以接收异步信息。`update()` 生命周期方法对于每个消息都会被调用。这使得组件可以根据消息的内容来更新自身，并决定是否需要重新渲染自己。消息可以由 HTML 元素监听器触发，或者由子组件，Agents，Services 或 Futures 发送。

`mounted()` 组件生命周期方法调用是在 `view()` 被处理并且 Yew 已经把组件挂载到 DOM 上之后，浏览器刷新页面之前。组件通常希望实现此方法以执行只能在组件渲染元素之后才能执行的操作。如果你想在做出一些更改后重新渲染组件，返回 `true` 就可以了。(不是必须要的)

change：组件可能被其父节点重新渲染。发生这种情况时，它们可以接收新的属性（properties）并选择重新渲染。这种设计通过更改属性（properties）来促进父子组件之间的通信。你不是必须实现 `change()`，但是如果想在组件被创建后通过 props 来更新组件，则可能要这么做。



`Component` trait 有两个关联类型：`Message` 和 `Properties`。



**ComponentLink API**

callback：注册一个回调，该回调将在执行时将消息发送到组件的更新机制



[wee_alloc](https://github.com/rustwasm/wee_alloc) 是一个比 Rust 二进制文件中通常使用的分配器还小得多的微型分配器。用这个分配器来替代默认的分配器将使 WASM 文件体积更小，但会牺牲速度和内存开销。

### 代码：

其他都是setStatus,只有client有waitStatus

java中File类的使用主要是遍历文件夹里内容啥的，std::fs里read_dir()这样的函数可以实现，所以直接把File类存成String即可

模块client实现:main函数启动后，先读取配置文件，再直接调用几个类里实现的静态方法来把几个模块的示例的静态数据初始化，然后在begin函数中启动网络链接模块中的控制链接线程与文件夹监控线程，把syn示例传递给其他线程，然后进入wait状态，当有线程出错时会修改status参数并唤醒主线程，使得其发现参数的修改并终止所有线程



### Java总结:

* synchronized关键字: 锁机制:加了锁的方法只能同时由一个线程使用

* `final`，多线程同时访问时只能读不能写，这些不变类也是线程安全的

* Java线程的状态转换图

* Thread()实例

  run(),start(),t.join(),t.interrupted,sleep(),isInterrupted()

  标志位:public volatile(关键字，表示线程间共享的变量) boolean running = true;

* 守护线程: 为其他线程服务；非守护线程都执行完毕后，虚拟机退出；

* 可重入锁：每获取一次锁，记录+1，每退出`synchronized`块，记录-1，减到0的时候，才会真正释放锁

* wait()方法必须在当前获取的锁对象上调用，这里获取的是`this`锁，因此调用this.wait()；因为wait()方法调用时，会释放线程获得的锁，wait()方法返回后，线程又会重新试图获得锁；

  对`this`锁对象调用`notify()`方法，这个方法会唤醒一个正在`this`锁等待的线程

  `notifyAll()`将唤醒所有当前正在`this`锁等待的线程

* File类





开会:

客户端和服务端间的通讯，以及web上的通讯其实都是用的socket，tcpstream。

只是说要在网页上显示信息，需要js，html等；客户端与服务端则只是传递数据即可，

实现文件的上传和提交





问题:

* 大的项目?网络上给的代码都比较简单？

* 明确我们前后端具体工具?
* 后端到前端的文件传递? 浏览器页面里实现文件下载或者上传?
* 纠删码还得改写
* 函数的错误处理
* crate结构如何组织

周六晚上

* WebAssembly的教程
* 数据库
* node.js,和Js
* 服务端实现
* 纠删码



前端框架: yew https://yew.rs/docs/v/zh_cn/，https://my.oschina.net/zengsai/blog/3163260

后端框架: actix-web（https://github.com/actix/actix，https://www.cnblogs.com/b612/articles/12026497.html）,rocket（https://rocket.rs/v0.4/guide/）

rust 开发完整的web程序:(yew+actix_web，但我还没复现成功) https://xiaozhuanlan.com/topic/2067148395



rocket尝试

actix尝试

js及node.js复习，了解呈现界面的机制(js或html?)以及把本地文件通过后端传递到前端，并让前端下载的机制

wasm学习：

数据库学习

任务布置: 

数据库学习及讲解(重要概念以及操作啥的)，把17级详细设计报告里关于数据库表单啥的，操作啥的弄清楚，记录并讲解(lyf,sym)

wasm书籍学习，思考如何把client 和 server 编译为Wasm，以及更多的js和Rust互调的情况(对每种情况做好记录)?因为目前来看，Node.js的使用，需要调用一些模块，但这在Rust中应该不太现实? 感觉仅仅是字符串的传递?考虑要怎么尝试。（qy,pqz,lyx)

Rocket文档研究，感觉有望和yew结合起来，实现和node.js一样的效果(但目前文件下载还没看到)，另外就是说对文件夹遍历并呈现在前端，还有跳转什么的一些细节功能的实现，用Rust的话还不太确定，不过node.js应该是可以的。(lyx)

问题:纠删码的矩阵转换，数据库，在非main里引用模块(lyf,sym)





把数据实现后在前端尝试

### Node.js

![](D:\科大\大二下\操作系统\结题报告与思考记录\web.JPG)

简单搭建一个web服务器，通过浏览器访问时返回html文件



Express 是一个简洁而灵活的 node.js Web应用框架, 提供了一系列强大特性帮助你创建各种 Web 应用，和丰富的 HTTP 工具。

使用 Express 可以快速地搭建一个完整功能的网站。

Express 框架核心特性：

- 可以设置中间件来响应 HTTP 请求。

- 定义了路由表用于执行不同的 HTTP 请求动作。

- 可以通过向模板传递参数来动态渲染 HTML 页面。

- **body-parser** - node.js 中间件，用于处理 JSON, Raw, Text 和 URL 编码的数据。

  ```css
  1. bodyParser.json(options): 解析json数据
  2. bodyParser.raw(options): 解析二进制格式(Buffer流数据)
  3. bodyParser.text(options): 解析文本数据
  4. bodyParser.urlencoded(options): 解析UTF-8的编码的数据。
  请求体解析后，解析值都会被放到req.body属性，内容为空时是一个{}空对象。
  ```

- **cookie-parser** - 这就是一个解析Cookie的工具。通过req.cookies可以取到传过来的cookie，并把它们转成对象。

- **multer** - node.js 中间件，用于处理 enctype="multipart/form-data"（设置表单的MIME编码）的表单数据。



基本概念

* Express 应用使用回调函数的参数： **request** 和 **response** 对象来处理请求和响应的数据(有一系列方法)

  ​	res.download(path [, filename] [, fn])，实现文件传送到客户端被下载

  ​	res.sendFile( __dirname + "/" + "index1.html" );将对应文件发送到浏览器，显示其内容

* 路由决定了由谁(指定脚本)去响应客户端请求。

  在HTTP请求中，我们可以通过路由提取出请求的URL以及GET/POST参数。

  eg:http://127.0.0.1:8081/list_user,http://127.0.0.1:8081/

* 静态文件

* GET与POST(后者更安全，适合密码输入)

* 文件上传

* cookie管理





### HTML

* HTML 表单用于搜集不同类型的用户输入。

  * <input type="text" name="first_name">定义用于文本输入的单行输入字段

  * radio: 单选按钮

  * 定义用于向表单处理程序（form-handler）提交表单的按钮。

    表单处理程序通常是包含用来处理输入数据的脚本的服务器页面。

    表单处理程序在表单的 *action* 属性中指定：action 属性定义在提交表单时执行的动作。

    <form action="http://127.0.0.1:8081/process_get" method="GET">


    First Name: <input type="text" name="first_name"> <br>
    
    Last Name: <input type="text" name="last_name">
    
    <input type="submit" value="Submit">
    
    </form>



### HTTP教程

HTTP协议（HyperText Transfer Protocol，超文本传输协议）；HTTP基于TCP/IP通信协议来传递数据（HTML 文件, 图片文件, 查询结果等）

##### http简介

* 工作原理:

  * HTTP协议工作于客户端-服务端架构上。浏览器作为HTTP客户端通过URL向HTTP服务端即WEB服务器发送所有请求。

    Web服务器有：Apache服务器，IIS服务器（Internet Information Services）等。

    Web服务器根据接收到的请求后，向客户端发送响应信息。

  ​       HTTP默认端口号为80，但是你也可以改为8080或者其他端口。

  * 注意:
    * HTTP是无连接：无连接的含义是限制每次连接只处理一个请求。服务器处理完客户的请求，并收到客户的应答后，即断开连接。采用这种方式可以节省传输时间。
    * HTTP是媒体独立的：这意味着，只要客户端和服务器知道如何处理的数据内容，任何类型的数据都可以通过HTTP发送。客户端以及服务器指定使用适合的MIME-type内容类型。
    * HTTP是无状态：HTTP协议是无状态协议。无状态是指协议对于事务处理没有记忆能力。缺少状态意味着如果后续处理需要前面的信息，则它必须重传，这样可能导致每次连接传送的数据量增大。另一方面，在服务器不需要先前信息时它的应答就较快。

##### http消息结构

HTTP是基于客户端/服务端（C/S）的架构模型，通过一个可靠的链接来交换信息，是一个无状态的请求/响应协议。

一个HTTP"客户端"是一个应用程序（Web浏览器或其他任何客户端），通过连接到服务器达到向服务器发送一个或多个HTTP的请求的目的。

一个HTTP"服务器"同样也是一个应用程序（通常是一个Web服务，如Apache Web服务器或IIS服务器等），通过接收客户端的请求并向客户端发送HTTP响应数据。

HTTP使用统一资源标识符（Uniform Resource Identifiers, URI）来传输数据和建立连接。

一旦建立连接后，数据消息就通过类似Internet邮件所使用的格式[RFC5322]和多用途Internet邮件扩展（MIME）[RFC2045]来传送。

* 客户端请求消息

  客户端发送一个HTTP请求到服务器的请求消息包括以下格式：请求行（request line）、请求头部（header）、空行和请求数据四个部分组成，

  请求头包括：user-Agent:产生请求的浏览器类型

  ​						Accept:客户端可识别的内容类型列表

  ​						Host：主机地址

  下图给出了请求报文的一般格式。

  ![](D:\科大\大二下\操作系统\6月图片\http请求格式.JPG)

  请求报文:前三行为请求行，其余部分称为request-header

  请求行之后是请求首部。首部常见的部分有如下几个：

  l Accept：请求的对象类型。如果是“/”表示任意类型，如果是指定的类型，则会变成“type/”。

  l Accept-Language：使用的语言种类。

  l Accept-Encording：页面编码种类。

  l Accept-Charset：页面字符集。

  l User-Agent：提供了客户端浏览器的类型和版本。

  l Host：连接的目标主机，如果连接的服务器是非标准端口，在这里会出现使用的非标准端口。

  l Connection：对于HTTP连接的处理，keep-alive表示保持连接，如果是在响应报文中发送页面完毕就会关闭连接，状态变为close。

  ![](D:\科大\大二下\操作系统\6月图片\请求报文.JPG)

* 服务器响应消息

  HTTP响应也由四个部分组成，分别是：状态行、消息报头、空行和响应正文。

  状态行给出了服务器的http版本，以及一个响应码（如404）。

![](D:\科大\大二下\操作系统\6月图片\服务器响应.JPG)

##### HTTP 请求方法

| 序号 | 方法    | 描述                                                         |
| :--- | :------ | :----------------------------------------------------------- |
| 1    | GET     | 请求指定的页面信息，并返回实体主体。                         |
| 2    | HEAD    | 类似于 GET 请求，只不过返回的响应中没有具体的内容，用于获取报头 |
| 3    | POST    | 向指定资源提交数据进行处理请求（例如提交表单或者上传文件）。数据被包含在请求体中。POST 请求可能会导致新的资源的建立和/或已有资源的修改。 |
| 4    | PUT     | 从客户端向服务器传送的数据取代指定的文档的内容。             |
| 5    | DELETE  | 请求服务器删除指定的页面。                                   |
| 6    | CONNECT | HTTP/1.1 协议中预留给能够将连接改为管道方式的代理服务器。    |
| 7    | OPTIONS | 允许客户端查看服务器的性能。                                 |
| 8    | TRACE   | 回显服务器收到的请求，主要用于测试或诊断。                   |
| 9    | PATCH   | 是对 PUT 方法的补充，用来对已知资源进行局部更新 。           |

##### http响应头信息

HTTP请求头提供了关于请求，响应或者其他的发送实体的信息。比较重要的就是Content-Type,cookie

| 应答头           | 说明                                                         |
| :--------------- | :----------------------------------------------------------- |
| Allow            | 服务器支持哪些请求方法（如GET、POST等）。                    |
| Content-Encoding | 文档的编码（Encode）方法。只有在解码之后才可以得到Content-Type头指定的内容类型。利用gzip压缩文档能够显著地减少HTML文档的下载时间。Java的GZIPOutputStream可以很方便地进行gzip压缩，但只有Unix上的Netscape和Windows上的IE 4、IE 5才支持它。因此，Servlet应该通过查看Accept-Encoding头（即request.getHeader("Accept-Encoding")）检查浏览器是否支持gzip，为支持gzip的浏览器返回经gzip压缩的HTML页面，为其他浏览器返回普通页面。 |
| Content-Length   | 表示内容长度。只有当浏览器使用持久HTTP连接时才需要这个数据。如果你想要利用持久连接的优势，可以把输出文档写入 ByteArrayOutputStream，完成后查看其大小，然后把该值放入Content-Length头，最后通过byteArrayStream.writeTo(response.getOutputStream()发送内容。 |
| Content-Type     | 表示后面的文档属于什么MIME类型。Servlet默认为text/plain，但通常需要显式地指定为text/html。由于经常要设置Content-Type，因此HttpServletResponse提供了一个专用的方法setContentType。 |
| Date             | 当前的GMT时间。你可以用setDateHeader来设置这个头以避免转换时间格式的麻烦。 |
| Expires          | 应该在什么时候认为文档已经过期，从而不再缓存它？             |
| Last-Modified    | 文档的最后改动时间。客户可以通过If-Modified-Since请求头提供一个日期，该请求将被视为一个条件GET，只有改动时间迟于指定时间的文档才会返回，否则返回一个304（Not Modified）状态。Last-Modified也可用setDateHeader方法来设置。 |
| Location         | 表示客户应当到哪里去提取文档。Location通常不是直接设置的，而是通过HttpServletResponse的sendRedirect方法，该方法同时设置状态代码为302。 |
| Refresh          | 表示浏览器应该在多少时间之后刷新文档，以秒计。除了刷新当前文档之外，你还可以通过setHeader("Refresh", "5; URL=http://host/path")让浏览器读取指定的页面。 注意这种功能通常是通过设置HTML页面HEAD区的＜META HTTP-EQUIV="Refresh" CONTENT="5;URL=http://host/path"＞实现，这是因为，自动刷新或重定向对于那些不能使用CGI或Servlet的HTML编写者十分重要。但是，对于Servlet来说，直接设置Refresh头更加方便。  注意Refresh的意义是"N秒之后刷新本页面或访问指定页面"，而不是"每隔N秒刷新本页面或访问指定页面"。因此，连续刷新要求每次都发送一个Refresh头，而发送204状态代码则可以阻止浏览器继续刷新，不管是使用Refresh头还是＜META HTTP-EQUIV="Refresh" ...＞。  注意Refresh头不属于HTTP 1.1正式规范的一部分，而是一个扩展，但Netscape和IE都支持它。 |
| Server           | 服务器名字。Servlet一般不设置这个值，而是由Web服务器自己设置。 |
| Set-Cookie       | 设置和页面关联的Cookie。Servlet不应使用response.setHeader("Set-Cookie", ...)，而是应使用HttpServletResponse提供的专用方法addCookie。参见下文有关Cookie设置的讨论。 |
| WWW-Authenticate | 客户应该在Authorization头中提供什么类型的授权信息？在包含401（Unauthorized）状态行的应答中这个头是必需的。例如，response.setHeader("WWW-Authenticate", "BASIC realm=＼"executives＼"")。 注意Servlet一般不进行这方面的处理，而是让Web服务器的专门机制来控制受密码保护页面的访问（例如.htaccess）。 |

##### HTTP状态码

当浏览者访问一个网页时，浏览者的浏览器会向网页所在服务器发出请求。当浏览器接收并显示网页前，此网页所在的服务器会返回一个包含HTTP状态码的信息头（server header）用以响应浏览器的请求。

HTTP状态码的英文为HTTP Status Code。

下面是常见的HTTP状态码：

- 200 - 请求成功
- 301 - 资源（网页等）被永久转移到其它URL
- 404 - 请求的资源（网页等）不存在
- 500 - 内部服务器错误

##### HTTP content-type

Content-Type（内容类型），一般是指网页中存在的 Content-Type，用于定义网络文件的类型和网页的编码，决定浏览器将以什么形式、什么编码读取这个文件，

Content-Type 标头告诉客户端实际返回的内容的内容类型。

常见的媒体格式类型如下：

- text/html ： HTML格式
- text/plain ：纯文本格式
- text/xml ： XML格式
- image/gif ：gif图片格式
- image/jpeg ：jpg图片格式
- image/png：png图片格式

以application开头的媒体格式类型：

- application/xhtml+xml ：XHTML格式
- application/xml： XML数据格式
- application/atom+xml ：Atom XML聚合格式
- application/json： JSON数据格式
- application/pdf：pdf格式
- application/msword ： Word文档格式
- application/octet-stream ： 二进制流数据（如常见的文件下载）
- application/x-www-form-urlencoded ： <form encType="">中默认的encType，form表单数据被编码为key/value格式发送到服务器（表单默认的提交数据的格式）

另外一种常见的媒体格式是上传文件之时使用的：

- multipart/form-data ： 需要在表单中进行文件上传时，就需要使用该格式



##### Cookie

cookie是一种类似缓存的机制，它保存在一个本地的文本文件中，其主要作用是在发送请求时将cookie放在请求首部中发送给服务器，服务器收到cookie后查找自己已有的cookie信息，确定客户端的身份，然后返回相应的页面，cookie的方便之处在于可以保持一种已登录的状态，例如：我们注册一个论坛，每次访问都需要进行填写用户名和密码然后登录。而使用了cookie后，如果cookie没有到达过期时间，那么我们只需在第一次登录时填写信息然后登录，以后的访问就可以省略这一步骤。

在HTTP协议中，cookie的交互过程是这样的：首先是三次握手建立TCP连接，然后客户端发出一个http request，这个request中不包含任何cookie信息。

当服务器收到这个报文后，针对request method作出响应动作，在响应报文的实体部分，加入了set-cookie段，set-cookie段中给出了cookie的id，过期时间以及参数path，path是表示在哪个虚拟目录路径下的页面可以读取使用该cookie，将这些信息发回给客户端后，客户端在以后的http request中就会将自己的cookie段用这些信息填充。

如果用户在连接中通过了服务器相应的认证程序，服务器会添加一个cdb_auth到set-cookie中，这个段表示了客户端的认证信息，而客户端以后在访问过程中也会将cdb_auth信息写入自己的cookie字段。服务器每次收到http request后读取cookie，然后根据cookie的信息返回不同的页面。例如，没有通过认证的客户端在request中不会有cdb_auth，因此服务器读取cookie后，不会将通过认证的客户端的页面返回给该客户端。

### Rocket

* Lifecycle：Rocket的主要任务是侦听传入的Web请求，将请求分配给应用程序代码，并将响应返回给客户端

  Routing（路由）, validation（验证方式）,processing（处理中）,response（响应）

  - A set of parameters to match an incoming request against.
  - A handler to process the request and return a response

* 路由(Routing):

  要匹配的参数包括静态路径，动态路径，路径段，表单，查询字符串，请求格式 说明符和主体数据

* 挂载(Mounting):

  ```rust
  rocket::ignite().mount("/hello", routes![hello, world]);
  ```

* Requests:（一些自定义类型的加入则需要分别对各个部分实现其特征）

  * methods：＃[post（“ /” ）]，\#[get("/hello1/<name>")]（get, put, post, delete, head, patch, or options）

  * dynamic paths

    ```rust
    #[get("/hello/<name>")]
    fn hello(name: &RawStr) -> String {
        format!("Hello, {}!", name.as_str())
    }
    ```

    * Multiple Segments：

    ```rust
    use rocket::response::NamedFile;
    
    #[get("/<file..>")]
    fn files(file: PathBuf) -> Option<NamedFile> {
        NamedFile::open(Path::new("static/").join(file)).ok()
    }
    ```

    url里/后的字符串会放入file这个参数中

  * Forwarding（含Default ranking)

    ```rust
    #[get("/user/<id>")]
    fn user(id: usize) { /* ... */ }
    
    #[get("/user/<id>", rank = 2)]
    fn user_int(id: isize) { /* ... */ }
    
    #[get("/user/<id>", rank = 3)]
    fn user_str(id: &RawStr) { /* ... */ }
    
    fn main() {
        rocket::ignite()
            .mount("/", routes![user, user_int, user_str])
            .launch();
    }
    ```

    即根据rank的值来对url依次判断user,user_int,user_str三个函数是否匹配(要看/<id>与参数类型是否匹配

    后者就id:Result<usize, &RawStr>来接收所有/user/...的url，匹配到是usize就成功，否则在函数里进行错误处理

  * Query Strings

    ```rust
    #[get("/hello?wave&<name>")]
    fn hello(name: &RawStr) -> String {
        format!("Hello, {}!", name.as_str())
    }
    ```

    访问形式：(均接收John)	

    `/hello?name=John&wave` (reordered)

    `/hello?name=John&wave&id=123` (extra segments)

    `/hello?id=123&name=John&wave` (reordered, extra segments)

    `/hello?name=Bob&name=John&wave` (last value taken)

    * Optional Parameters

      ```rust
      #[get("/hello?wave&<name>")]
      fn hello(name: Option<String>) -> String {
          name.map(|name| format!("Hi, {}!", name))
              .unwrap_or_else(|| "Hello!".into())
      }
      ```

      url里可以要或不要name参数Multiple Segments

    * Multiple Segments

      ```rust
      use rocket::request::Form;
      
      #[derive(FromForm)]
      struct User {
          name: String,
          account: usize,
      }
      
      #[get("/item?<id>&<user..>")]
      fn item(id: usize, user: Form<User>) { /* ... */ }
      ```

      ```
      匹配:/item?id=100&name=sandal&account=400`, 
      传入参数:  
      id: 100 ; user : User { name: "sandal", account: 400 }
      ```

  * Request Guards: Rocket将自动为Request Guards调用实现。Rocket仅在其所有guards通过后才将请求调度到处理程序

    * Custom Guards

      您可能还实现`FromRequest`了`AdminUser`使用传入cookie对管理员进行身份验证的类型。然后，确保在其参数列表中具有`AdminUser`或`ApiKey`类型的任何处理程序仅在满足适当条件的情况下才被调用

      ```
      #[get("/sensitive")]
      fn sensitive(key: ApiKey) { /* .. */ }
      ```

    * Guard Transparency

      作为一个具体示例，以下应用程序具有函数，该函数`health_records`返回数据库中的所有运行状况记录。因为健康记录是敏感信息，所以只有超级用户才能访问它们。所述`SuperUser`请求后卫认证和授权超级用户，它的`FromRequest`实现是通过它的唯一手段`SuperUser`可以构造。通过声明以下`health_records`功能，可以确保在*编译时*防止违反健康记录的访问控制：

      ```
      fn health_records(user: &SuperUser) -> Records { /* ... */ }
      ```

    * Forwarding Guards(Request guards和forwarding的概念的结合)

      ```rust
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

      不过AdminUser和User的实现？

  * Cookies（不太懂

  * Format:指定它愿意接受或响应的数据格式。参数的值是一个字符串，用于标识HTTP媒体类型或简写形式

    ```rust
    #[post("/user", format = "application/json", data = "<user>")]//或简写:format = "json"
    fn new_user(user: User) { /* ... */ }
    ```

    post时: only incoming requests with hearder`Content-Type: application/json` will match `new_user`

    而GET，HEAD，OPTION时:匹配的是`Accept` header

  * Body Data:To indicate that a handler expects body data, annotate it with `data = "<param>"`, where `param` is an argument in the handler

    ```rust
    #[post("/", data = "<input>")]
    fn new(input: T) { /* .. */ }//T实现了the FromData trait
    ```

    * Forms(表单data处理)//此例中表单包含一个复选框和一个文本段

      ```rust
      use rocket::request::Form;
      
      #[derive(FromForm)]
      struct Task {
          complete: bool,//checkbox
          description: String,//text filed
      }
      
      #[post("/todo", data = "<task>")]
      fn new(task: Form<Task>) { /* .. */ }
      //fn new(task: Option<Form<Task>>) { /* .. */ }错误处理需要:Option或Result类型包装
      ```

      Form<Task>：The [`Form`](https://api.rocket.rs/v0.4/rocket/request/struct.Form.html) type implements the `FromData` trait as long as its generic parameter implements the [`FromForm`](https://api.rocket.rs/v0.4/rocket/request/trait.FromForm.html) trait。而在例子汇总，已经在Task结构体中自动生成了该trait。`FromForm` can be derived for any structure whose fields implement `FromFormValue`

      * Lenient Parsing： if an incoming form contains the fields "a", "b", and "c" while `T` only contains "a" and "c", the form *will* parse as `LenientForm<T>`,即可以允许request里传入的参数超过函数的argument里参数需要(前面的Form则不行，必须完全一致)

        用法、规则与Form完全一致：

        `fn new(task: LenientForm<Task>) { /* .. */ }`

      * Field Renaming:重命名:rust结构体里数据名可以和传入的from字段的名称不一致了

        ```rust
        #[derive(FromForm)]
        struct External {
            #[form(field = "type")]
            api_type: String
        }
        ```

      * Field Validation：Fields of forms can be easily validated via implementations of the [`FromFormValue`](https://api.rocket.rs/v0.4/rocket/request/trait.FromFormValue.html) trait

        ```rust
        use rocket::http::RawStr;
        use rocket::request::FromFormValue;
        
        struct AdultAge(usize);
        
        impl<'v> FromFormValue<'v> for AdultAge {
            type Error = &'v RawStr;
        
            fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
                match form_value.parse::<usize>() {
                    Ok(age) if age >= 21 => Ok(AdultAge(age)),
                    _ => Err(form_value),
                }
            }
        }
        
        #[derive(FromForm)]
        struct Person {
            age: Adult//age: Option<AdultAge>
        }
        ```

    * JSON:

      ```rust
      use serde::Deserialize;
      use rocket_contrib::json::Json;
      
      #[derive(Deserialize)]
      struct Task {
          description: String,
          complete: bool
      }
      
      #[post("/todo", data = "<task>")]
      fn new(task: Json<Task>) { /* .. */ }
      ```

      * Streaming

        用data type来实现流形式IO：

        ```rust
        use rocket::Data;
        
        #[post("/upload", format = "plain", data = "<data>")]
        fn upload(data: Data) -> Result<String, std::io::Error> {
            data.stream_to_file("/tmp/upload.txt").map(|n| n.to_string())
        }
        ```

        上面的路由接受`POST`对`/upload`路径的任何请求（需`Content-Type: text/plain`)。传入的数据将流传输到`tmp/upload.txt`，如果上传成功，则写入的字节数将作为纯文本响应返回

  * Error Catchers:错误捕捉,针对错误状态码，catchers与routes类似，但也有一定区别

    ```rust
    use rocket::Request;
    #[catch(404)]
    fn not_found(req: &Request) -> String {
        format!("Sorry, '{}' is not a valid path.", req.uri())
    }//参数必须为&Request(只能是0或1个参数)
    fn main() {
        rocket::ignite().register(catchers![not_found]);
    }
    ```

* Responses:实现了Responder trait 的类型均可以作为handler的返回值

  * Responder：A `Response` includes an HTTP status, headers, and body. The body may either be *fixed-sized* or *streaming*（取决于具体类型:如String为 *fixed-sized* ，File为streaming）

    * Wrapping:可以对responder进行包装：status,content两个模块分别用于包装status code和Content-Type

      ```rust
      use rocket::response::status;
      
      #[post("/<id>")]
      fn new(id: usize) -> status::Accepted<String> {
          status::Accepted(Some(format!("id: '{}'", id)))
      }
      ```

    * Errors:如果已为给定的状态码注册了一个错误捕获器，则Rocket将调用它。捕获器创建并向客户端返回响应。如果未注册任何错误捕获器，并且错误状态代码是标准HTTP状态代码之一，则将使用默认错误捕获器。默认错误捕获器返回带有状态代码和描述的HTML页面。如果没有用于自定义状态代码的捕获器，Rocket将使用**500**错误捕获器返回响应。

    * Status：可以直接把一个request转到一个error cathcer(根据其status code)（此处总结了status code与其对应的catcher）

      ```rust
      use rocket::http::Status;
      
      #[get("/")]
      fn just_fail() -> Status {
          Status::NotAcceptable
      }
      ```

  * Custom Responders：（自定义Responder）：不太理解，还需对request和reponse的信息做一定研究

    if your custom responder wraps an existing responder, headers, or sets a custom status or content-type, `Responder` can be automatically derived

  * Implementation:已经实现了Responder trait的:`String`, `&str`, `File`, `Option`, and `Result`

    * Strings: &str和String（有个implementation示例），所以handler函数里可以返回&str或String类型数据

      ```rust
      #[get("/string")]
      fn handler() -> &'static str {
          "Hello there! I'm a string!"
      }
      ```

    * Option:an `Option<T>` can only be returned when `T` implements `Responder`

      This implementation makes `Option` a convenient type to return when it is not known until process-time whether content exists. For example, because of `Option`, we can implement a file server that returns a `200` when a file is found and a `404` when a file is not found in just 4, idiomatic lines:

      ```rust
      use rocket::response::NamedFile;
      
      #[get("/<file..>")]
      fn files(file: PathBuf) -> Option<NamedFile> {
          NamedFile::open(Path::new("static/").join(file)).ok()
      }
      ```

    * Result:depends on whether the error type `E` implements `Responder`；

      When the error type `E` implements `Responder`, the wrapped `Responder` in `Ok` or `Err`

      If the error type `E` *does not* implement `Responder`, then the error is simply logged to the console, using its `Debug` implementation, and a `500` error is returned to the client

      ```rust
      use rocket::response::NamedFile;
      use rocket::response::status::NotFound;
      
      #[get("/<file..>")]
      fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
          let path = Path::new("static/").join(file);
          NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
      }
      ```

      与Option区别在于，Result在file没找到时能返回更多自定义信息

  * Rocket Responders

    * [`Content`](https://api.rocket.rs/v0.4/rocket/response/struct.Content.html) - Used to override the Content-Type of a response.

    * [`NamedFile`](https://api.rocket.rs/v0.4/rocket/response/struct.NamedFile.html) - Streams a file to the client; automatically sets the Content-Type based on the file's extension.

    * [`Redirect`](https://api.rocket.rs/v0.4/rocket/response/struct.Redirect.html) - Redirects the client to a different URI.

    * [`Stream`](https://api.rocket.rs/v0.4/rocket/response/struct.Stream.html) - Streams a response to a client from an arbitrary `Read`er type.当需要返回大量数据到client时优选，因为可以节省大量内存。The `Stream` type can be created from any `Read` type

      ```rust
      use std::os::unix::net::UnixStream;
      use rocket::response::Stream;
      
      #[get("/stream")]
      fn stream() -> Result<Stream<UnixStream>, std::io::Error> {
          UnixStream::connect("/path/to/my/socket").map(Stream::from)
      }
      ```

    * [`status`](https://api.rocket.rs/v0.4/rocket/response/status/) - Contains types that override the status code of a response.

    * [`Flash`](https://api.rocket.rs/v0.4/rocket/response/struct.Flash.html) - Sets a "flash" cookie that is removed when accessed.

    * [`Json`](https://api.rocket.rs/v0.4/rocket_contrib/json/struct.Json.html) - Automatically serializes values into JSON.

      返回 type `Json<T>` ，其中 `T` 是要被 serialize into JSON的结构体. The type `T` must implement the [`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) trait from [`serde`](https://docs.serde.rs/serde/), 这个可以被自动derived.

      ```rust
      use serde::Serialize;
      use rocket_contrib::json::Json;
      
      #[derive(Serialize)]
      struct Task { /* .. */ }
      
      #[get("/todo")]
      fn todo() -> Json<Task> {
          Json(Task { /* .. */ })
      }
      ```

      The Json type serializes the structure into JSON, sets the Content-Type to JSON, and emits the serialized data in a fixed-sized body. If serialization fails, a 500 - Internal Server Error is returned.

    * [`MsgPack`](https://api.rocket.rs/v0.4/rocket_contrib/msgpack/struct.MsgPack.html) - Automatically serializes values into MessagePack.

    * [`Template`](https://api.rocket.rs/v0.4/rocket_contrib/templates/struct.Template.html) - Renders a dynamic template using handlebars or Tera.

  * Templates：

    ```rust
    use rocket_contrib::templates::Template;
    
    #[get("/")]
    fn index() -> Template {
        let context = /* object-like value */;
        Template::render("index", &context)
    }
    ```

    The context 可以是任何实现了  `Serialize` and serializes into an `Object` value的类型， 如 structs, `HashMaps`, and others.

    要render，先要register：

    ```rust
    fn main() {
        rocket::ignite()
            .mount("/", routes![/* .. */])
            .attach(Template::fairing());
    }
    ```

    Rocket在可配置 `template_dir`目录中发现模板。Rocket中的模板支持与引擎无关。用于呈现模板的引擎取决于模板文件的扩展名。例如，如果文件以结尾`.hbs`，则使用Handlebars；如果文件以结尾`.tera`，则使用Tera。

    模板的名称*不*包括其扩展名。对于命名模板文件`index.html.tera`，调用`render("index")`和使用名称`"index"`的模板，即`{% extends "index" %}`或`{% extends "base" %}`为`base.html.tera`。

    * Live Reloading：更多例子链接

  * Typed URIs：这一块不是很懂在干嘛?在handler里用uri! 宏来创建route？

    ```rust
    let mike = uri!(person: age = 28, name = "Mike");
    assert_eq!(mike.to_string(), "/person/Mike?age=28");
    ```

* State：许多Web应用程序都需要维护状态。这可以像维护访问次数计数器一样简单，也可以像需要访问作业队列和多个数据库一样复杂

  * Managed State

    The process for using managed state is simple：

    1. Call `manage` on the `Rocket` instance corresponding to your application with  the state的初始值.
    2. 将`State<T>`类型添加到任何请求处理程序中，其中`T`是传递给的值的类型`manage`。

    由于Rocket自动为您的应用程序提供多线程，因此处理程序可以同时访问Managed State。所以Managed State必须是线程安全的。 Thanks to Rust, this condition is checked at compile-time by ensuring that the type of values you store in managed state implement `Send` + `Sync`.

    * Adding State:eg:让Rocket manage一个结构体(HitCount，有count初值为0)

      ```rust
      use std::sync::atomic::AtomicUsize;
      
      struct HitCount {
          count: AtomicUsize
      }
      
      rocket::ignite().manage(HitCount { count: AtomicUsize::new(0) });
      ```

      ```rust
      //可以调用manage多次(每次的value是不同type)
      rocket::ignite()
          .manage(HitCount { count: AtomicUsize::new(0) })
          .manage(Config::from(user_input));
      ```

    * Retrieving State

      ```rust
      use rocket::State;
      
      #[get("/count")]
      fn count(hit_count: State<HitCount>) -> String {//通过count函数的参数可以直接得到其managed state的值
          let current_count = hit_count.count.load(Ordering::Relaxed);
          format!("Number of visits: {}", current_count)
      }
      //也可以在一个handler中同时得到多个managed state
      #[get("/state")]
      fn state(hit_count: State<HitCount>, config: State<Config>) { /* .. */ }
      ```

    * Within Guards:不太理解，可能和前面Guards结合

  * Request-Local State：request-local state is *local* to a given request，即该state的存储只限于request到来到完成的期间

    Request-local state is *cached*: if data of a given type has already been stored, it will be reused

    ```rust
    use rocket::request::{self, Request, FromRequest};
    
    /// A global atomic counter for generating IDs.
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    
    /// A type that represents a request's ID.
    struct RequestId(pub usize);
    
    /// Returns the current request's ID, assigning one only as necessary.
    impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
        type Error = ();
    
        fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
            // The closure passed to `local_cache` will be executed at most once per
            // request: the first time the `RequestId` guard is used. If it is
            // requested again, `local_cache` will return the same value.这段注释理解很关键，但还没懂透
            request::Outcome::Success(request.local_cache(|| {
                RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
            }))
        }
    }
    
    #[get("/")]
    fn id(id: &RequestId) -> String {
        format!("This is request #{}.", id.0)
    }
    ```

    这个例子把ID和每个request相关联(即每个request有个对应的ID)

  * Databases: 使用r2d2库提供的连接池

    三步连接到数据库：

    1. Configure the databases in `Rocket.toml`.
    2. Associate a request guard type and fairing with each database.
    3. Use the request guard to retrieve a connection in a handler.

    Rocket支持的数据库

    | Kind     | Driver                                                       | Version | `Poolable` Type                                              | Feature                |
    | -------- | ------------------------------------------------------------ | ------- | ------------------------------------------------------------ | ---------------------- |
    | MySQL    | [Diesel](https://diesel.rs/)                                 | `1`     | [`diesel::MysqlConnection`](http://docs.diesel.rs/diesel/mysql/struct.MysqlConnection.html) | `diesel_mysql_pool`    |
    | MySQL    | [`rust-mysql-simple`](https://github.com/blackbeam/rust-mysql-simple) | `16`    | [`mysql::conn`](https://docs.rs/mysql/14.0.0/mysql/struct.Conn.html) | `mysql_pool`           |
    | Postgres | [Diesel](https://diesel.rs/)                                 | `1`     | [`diesel::PgConnection`](http://docs.diesel.rs/diesel/pg/struct.PgConnection.html) | `diesel_postgres_pool` |
    | Postgres | [Rust-Postgres](https://github.com/sfackler/rust-postgres)   | `0.15`  | [`postgres::Connection`](https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html) | `postgres_pool`        |

    数据库这一块了解得还是不够，也没有安装尝试

* Fairings：处理结构化中间件的方法，使您的应用程序可以进入request的生命周期，以记录或重写有关传入请求和传出响应的信息（ rewriting requests or responses, recording information about the event, or doing nothing at all.）

  * Attaching：需要先注册Fairings(attach方法）

    ```rust
    rocket::ignite()
        .attach(req_fairing)
        .attach(res_fairing)//按attach的顺序依次指向执行
        .launch();
    ```

  * Callbacks：There are four events for which Rocket issues(发出) fairing callbacks

    - **附加（`on_attach`）**

      首先通过该[`attach`](https://api.rocket.rs/v0.4/rocket/struct.Rocket.html#method.attach)方法附加整流罩时，将调用Attach回调。Attach回调可以任意修改`Rocket`正在构造的实例，并可以选择中止启动。附加整流罩通常用于解析和验证配置值，中止不良配置以及将解析后的值插入托管状态以供以后检索。

    - **启动（`on_launch`）**

      在Rocket应用程序启动之前立即调用启动回调。启动回调可以检查`Rocket`正在启动的实例。启动回调可以是与正在启动的Rocket应用程序相关的启动服务的便捷挂钩。

    - **要求（`on_request`）**

      收到请求后立即调用请求回调。请求回调可以随意修改请求，并查看传入的数据。但是，它可能不会中止或直接响应该请求；这些问题可以通过请求防护或响应回调更好地处理。

    - **回应（`on_response`）**

      准备将响应发送给客户端时，将调用响应回调。响应回调可以修改部分或全部响应。这样，当较大的应用程序失败时，可以通过根据需要重写**404**响应来使用响应整流罩来提供响应。作为另一个示例，响应整流罩也可以用于将标头注入所有传出响应中。

  * Implementing：a fairing is any type that implements the [`Fairing`](https://api.rocket.rs/v0.4/rocket/fairing/trait.Fairing.html) trait

    * 必须实现info方法：to assign a name to the fairing and determine the set of callbacks the fairing is registering for。 A `Fairing` can implement any of the available callbacks（均为method）: [`on_attach`](https://api.rocket.rs/v0.4/rocket/fairing/trait.Fairing.html#method.on_attach), [`on_launch`](https://api.rocket.rs/v0.4/rocket/fairing/trait.Fairing.html#method.on_launch), [`on_request`](https://api.rocket.rs/v0.4/rocket/fairing/trait.Fairing.html#method.on_request), and [`on_response`](https://api.rocket.rs/v0.4/rocket/fairing/trait.Fairing.html#method.on_response)

    * Requirements：Send + Sync + 'static

    * Example：在每一个请求（get或post）来时计数器加1，并在get访问/counts 路由时返回当前的计数值

      ```rust
      use std::io::Cursor;
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      use rocket::{Request, Data, Response};
      use rocket::fairing::{Fairing, Info, Kind};
      use rocket::http::{Method, ContentType, Status};
      
      struct Counter {
          get: AtomicUsize,
          post: AtomicUsize,
      }
      
      impl Fairing for Counter {
          // This is a request and response fairing named "GET/POST Counter".
          fn info(&self) -> Info {
              Info {
                  name: "GET/POST Counter",
                  kind: Kind::Request | Kind::Response
              }
          }
      
          // Increment the counter for `GET` and `POST` requests.
          fn on_request(&self, request: &mut Request, _: &Data) {
              match request.method() {
                  Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
                  Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
                  _ => return
              };
          }
      
          fn on_response(&self, request: &Request, response: &mut Response) {
              // Don't change a successful user's response, ever.
              if response.status() != Status::NotFound {
                  return
              }
      
              // Rewrite the response to return the current counts.
              if request.method() == Method::Get && request.uri().path() == "/counts" {
                  let get_count = self.get.load(Ordering::Relaxed);
                  let post_count = self.post.load(Ordering::Relaxed);
                  let body = format!("Get: {}\nPost: {}", get_count, post_count);
      
                  response.set_status(Status::Ok);
                  response.set_header(ContentType::Plain);
                  response.set_sized_body(Cursor::new(body));
              }
          }
      }
      ```

    * Ad-Hoc Fairings：简化对Fairing trait的实现

  * Testing：

    * Local Dispatching+Validating Responses

      ```rust
      use rocket::local::Client;
      use rocket::http::{ContentType, Status};
      
      let rocket = rocket::ignite().mount("/", routes![hello]);
      let client = Client::new(rocket).expect("valid rocket instance");
      let mut response = client.get("/").dispatch();
      
      assert_eq!(response.status(), Status::Ok);
      assert_eq!(response.content_type(), Some(ContentType::Plain));
      assert!(response.headers().get_one("X-Special").is_some());
      assert_eq!(response.body_string(), Some("Expected Body".into()));
      ```

      类似于创建client示例来发出请求，然后再assert_eq!看与预期是否一致

    * ```rust
      //Hello world的完整测试
      #![feature(proc_macro_hygiene, decl_macro)]
      #[macro_use] extern crate rocket;
      
      #[get("/")]
      fn hello() -> &'static str {
          "Hello, world!"
      }
      
      fn rocket() -> rocket::Rocket {
          rocket::ignite().mount("/", routes![hello])
      }
      
      fn main() {
          rocket().launch();
      }
      
      #[cfg(test)]
      mod test {
          use super::rocket;
          use rocket::local::Client;
          use rocket::http::Status;
      
          #[test]
          fn test_hello() {
              let client = Client::new(rocket()).unwrap();
              let mut response = client.get("/").dispatch();
              assert_eq!(response.status(), Status::Ok);
              assert_eq!(response.body_string(), Some("Hello, world!".into()));
          }
      }
      ```

    * Codegen Debug：`ROCKET_CODEGEN_DEBUG=1 cargo build`

      得到如

      ```
      note: emitting Rocket code generation debug output
       --> examples/hello_world/src/main.rs:7:1
        |
      7 | #[get("/")]
        | ^^^^^^^^^^^
        |
        = note:
          fn rocket_route_fn_hello<'_b>(
              __req: &'_b ::rocket::Request,
              __data: ::rocket::Data
          ) -> ::rocket::handler::Outcome<'_b> {
              let responder = hello();
              ::rocket::handler::Outcome::from(__req, responder)
          }
      ```

* Pastebin项目:

  * 开始构建:即创建工程，添加dependency啥的

  * index: get "/" :返回字符串，即根网址显示操作说明；Rocket will take the string and return it as the body of a fully formed HTTP response with `Content-Type: text/plain`

  * uploading:

    * 创建unique IDs:结构体`pub struct PasteID<'a>(Cow<'a, str>);`并实现了trait:`fmt::Display`和`FromParam<'a>`

    * Upload Route：

      1. 创建`PasteId`您选择的新长度。
      2. 在`upload/`给定的内部构造一个文件名`PasteId`。
      3. 将传输`Data`到具有构造文件名的文件。
      4. 根据构造URL `PasteId`。
      5. 将URL返回给客户端。

    * Retrieving Pastes:给定一个`<id>`，将返回相应的粘贴（如果存在）(即在浏览器端访问`GET /<id>` route，则会返回相应的文件内容)

      需要对访问的url的id做判断，看其所指的文件是否valid（即允许访问)

      To prevent the attack, we need to *validate* `id` before we use it. Since the `id` is a dynamic parameter, we can use Rocket's [FromParam](https://api.rocket.rs/v0.4/rocket/request/trait.FromParam.html) trait to implement the validation and ensure that the `id` is a valid `PasteId` before using it

      FromParam理解参见https://api.rocket.rs/v0.4/rocket/request/trait.FromParam.html的开头和Forwarding段落的阅读





问题: 请求怎么发? 加cookie或数据，或各种header啥的还不太会，得查查资料(或许postman可以，但还不太会用)



客户端和服务端的调试和完善，main函数模块的实现

Rocket遇到的问题与思考

数据库知识与搭建

我还有html，还有一些前端概念
感觉看了rocket后还是有点懵
很多概念都一知半解
而且他上面给的很多例子
都只给了一部分没实现完的，或者说我不太懂的部分
跟http请求有关
搞得后面一些概念的例子我也没法运行来尝试
就没理解到位



看了rocket文档

目前需要注意的是wasm的结合这一块，自己还没想清楚

以及bug调试



### 前端搭建构思

* 利用Tomcat Web服务程序搭建动态网站（采用MVC设计模式，控制器从视图读取数据，控制用户输入，向模型发送数据管理复杂的应用程序，可以在一个时间内专门关注一个方面）
* 响应前端请求，调用服务器端Java程序访问数据库和进行文件操作，返回动态数据
* BOOTSTRAP主题 打造优美交互界面
* Jqery + AJAX异步C/S通讯+JSON实现文件目录层次的交互式刷新，响应用户网页操作
* 基本思路: 前端html -> 通过按钮点击，进行ajax响应，传递数据到java函数进行操作，再传回来，最后相应更新浏览器页面



文件结构：

* index.html:让网页端用户进行注册和登录(即根界面)

* index_ajax.js:让网页具有和用户和服务器交互的动态能力。 即为注册和登录的button提供js动作，调用异步的 ajax 将表单信息提交给服务器（没有用submit提交，因为这样会导致页面刷新，不符合预期，使用ajax实现页面部分刷新）并调用下面两个 java 程序之一进行服务（参见对应文件）

  eg：form 格式的 var 来进行 data 的传送，采用 post 方法，在回调函数中更改网页 html，输出服务器反馈信息 

* 网页主界面:

  是用户登录后的主界面，在这个界面用户可以看到我们项目的介绍图片，标题，介绍，最重要的是可以访问系统的文件目录，可以进入某个文件夹或者返回上层目录，还可以进行文件的下载上传重命名等访问和管理

  * majorPage.jsp：界面主要的html代码。采用 jsp 在服务器端**动态**生成 html 代码，因为第一次打开该网页就会展示文件系统根目录文件夹信息，这些信息是动态的。因此我动态的查询数据库并返回 html 代码。

    但注意到因为是jsp，所以可以插入java代码动态执行(即每次打开该页面都要查询数据库得到动态数据)

  * majorPage_ajax.js: 包含了主界面全部的用户交互的代码（原代码有不少注释），即对按钮进行相应(下载及进度条显示(定时刷新)，进入子目录，返回上一级目录等)

* 文件目录展示交互模块

  * GetFileList.java------根据输入：查询的全路径；输出该路径下的全部列表项的 html 代码。 该类使用了数据库访问包： import database.*;
  * majorPage_ajax.js------包含该模块主要代码

* 文件下载模块:该模块提供了用户选中单个文件并进行下载的全套服务。 用户选中单个文件，点击下载，服务器开始收集碎片，实时反馈进度，网页进度条实时更新，进度 100%后可单击进度条下载该文件

  当用户点击下载后，遍历列表，对于每一个勾选项进行下载操作。 

  下载操作就是： 

  获取要下载文件全路径和名称； 

  利用 ajax 调用动态方法 FileDownloader!downloadRegister，请求服务器收集碎片； 

  问该文件任务添加进度条； 

  定时通过 ajax 调用动态方法 FileDownloader!progressCheck 检测收集进度，并刷新进度条。

  如果进度到达 100%，利用 ajax 调用动态方法 FileDownloader!decodeFile 进行碎片

  远程拼接，为进度条添加下载属性，链接到生成的要下载的文件。

  * FileDownloader.java

    * downloadRegister方法:将一条下载请求插入数据库，让服务器将知道要从各个客户端收集该指定的碎片，调用了database包（利用ａｊａｘ　远程调用　downloadRegister(String path, String name); action形式)

    * progressCheck：服务器查询特定本地临时碎片数目，计算出碎片收集进度并且返回给网页。 该函数调用了本地文件访问接口。 

    * decodeFile() :预下载完成后，即判定系统中碎片已经足够时调用

      服务器调用 erasurecode 开源解码程序，将特定文件复原，等待用户通过 http 请求下载。 

  * majorPage_ajax.js

* userManagement包（对之前完成的database和纠删码的包有大量调用）

  注意到包里的类都extends了ActionSupport,使其成为了tomcat的可调用程序。所以ajax调用如GetFileList.action可以直接执行GetFileList类的excute函数，该函数可以返回字符串如html(返回不是return，而是对该类实现set和get方法即可，这样在ajax代码里可以通过var obj来调用)

  * UserLogin.java：接受来自网页的登录请求，查询数据库进行身份核实和反馈核实结果。

    通过继承 extends ActionSupport，使得该 java 成为 tomcat 的可调用程序。 只要帮 class 内部的变量提供了设置（set）和返回（get）函数，该变量就会自动被返回到 js，通过 obj.X 的格式访问该变量，X 为其在类中的名字

    我们从 js 接收用户名，根据用户名查询服务器客户端，获得正确密码，再和用户提交的密码对比，一致则登陆成功，跳转到主界面，否则登陆失败，返回失败信息到网页

    在struts中配置，使得返回json对象而非跳转的html

  * UserReg.java：接收来自网页的用户注册请求，将请求插入数据库等待管理员审核（管理员审核?），基本同UserLogin.java

  * GetFileList.java

  * FileDownloader.java

    

  

需要考虑实现的部分：（动态数据的返回？）

1. 前端登录注册模块
2. 文件目录层次的交互式刷新，响应用户对文件目录的操作
3. 下载操作的实现
4. 界面优化



登录注册界面

登录成功后的主界面：开始就会jsp文件来读取数据库，然后把当前的文件目录展示出来；然后通过ajax对文件目录动态加载，具体就是在点击文件夹时，响应该事件，调用java函数来查询该文件夹目录下的文件，并以字符串形式返回html，然后实现交互式刷新

下载：点击下载按钮，就请求服务器收集碎片



登录注册模块:考虑接受来自网页的登录请求，查询数据库进行身份核实和反馈核实结果。注册也同理

* 需要解决的问题：文件目录的显示和子目录刷新？（原来是用jsp动态显示)：考虑还是改写成ajax响应：挂载到登录按钮上

  这样还是可以通过struts+Tomcat来运行

* 然后后端就还是Rocket来响应，还是完善GetFilelist和FileDownloader模块，再写写Rocket的接口即可

  不过还需调研一下ajax的那个设置

* 接口：

  * UserReg.execute(params.userName,params.userPasswd);成功返回字符串"success",失败返回字符串"fail"

  * UserLogin.execute(params.userName,params.userPasswd); 成功返回字符串"login sucessfully!"，失败返回字符串"login fail!"

  * FileDownloader:

    * downloadRegister:  FileDownloader::downloadRegister(params.path,params.name);

      返回值同java文件所述："NotEnoughFragments"或"Error"或“OK”

    * progressCheck： FileDownloader::progressCheck(params.path,params.name);

      返回值:"Error"或者进度的数值的字符串形式

    * decodeFile：FileDownloader::decodeFile(params.path,params.name); 返回“Error”或“OK”

  * FileDownloader::GetFileList.execute(params.QueryPath); 返回值为字符串html(后续再做错误处理)



项目框架：

client和server：客户端运行，把文件放进指定文件夹，然后会被分块，传输给服务端，服务端则会把碎片分发给各个客户端，然后在数据库里做相应记录

前后端：本来是打算采用Node.js来写，但是后续调研发现WebAssembly编译Rust存在较大的局限性，所以整个项目编译为WebAssembly是不现实的，主要还是在前端做一定的编译。然后随后调研，查阅资料后发现，考虑到17级已经实现了比较完整的前后端。那么我们初步设定目标就是写好client和server

然后后续为了和我们的项目契合，以及统一项目工程，是考虑的逐步用rust取代原来的Tomcat服务器为中心构建的前后端。

目前我们是考虑在前后端分离，采用actix_web框架实现后端，采用Seed实现前端。目前后端工作基本完成，还在调试阶段。主要考虑的是对采用Tomcat启动的前端的事件作ajax响应，把其发送到后端，然后后端和数据库做交互，最后把结果返回给前端

前端这一块，考虑用seed框架来完成，其内置了把rust编译为webAssembly的几个包，是比较理想的完成方式，但是由于前端开始的相对较晚(前期准备不充分，低估了rust写前端的难度)，所以目前进度不算理想，可能考虑后续完善。



锁机制：先是创建锁（其包含一个状态值），并分别clone到folderScanner和ServerConnecter两个线程；然后主线程调用wait进入睡眠；最后若另外两个线程发生错误，则会获取锁并修改状态值，再唤醒主线程；主线程检测到状态值改变，则会输出相应报错信息，最后终止所有client的线程

整个从server，client，到前后端的实现的构思



what why how

* 项目的架构：整体的展示。 （图）

* 分部分介绍和亮点：和数据库交互的模块，接口实现；

  * client

  * server

    考虑前后端分离，然后分别采用两个rust框架来实现。

  * 后端actix-web

  * 前端seed

* 意义价值、愿景（多少年以后）

  * rust+webAssembly的前沿性和优势（安全性，rust机制）

  * 完整项目的缺失，需要来实践这么一个项目。

  * 为rust和webAssembly开源社区做贡献

  * 项目初衷(17级  详细设计报告和答辩报告（创新计划和答辩报告）)

  * 调研报告：低功耗、低内存占用；闲置系统，极其受限

  * 分布式在线服务；

  * 我们的调研和可行性报告

  * 稳定性？怎么测试

* 与现有系统的比较，他们做到了什么程度，有什么优缺点，资源浪费，我们又有什么思路来完成；

  现有项目存在的问题：资源浪费、效率低下等；现有相关工作？我们的思路、可能遇到的挑战、对挑战的克服方法、项目愿景和可能的贡献；

  * 将分布式文件系统：17级gfs等

  * 17级项目：rust，低功耗

  * 我们思路：rust实现（优势），WebAssembly，整体用rust来实践

    遇到的挑战：写client和server的时候遇到的挑战：

  * 愿景和贡献：实现一个完全以rust和WebAssembly来成的完整的分布式文件系统并部署到公网。

* demo：想想问点什么东西？：rust学习曲线；rsut前端确实还未完善，本身一些功能还没有很好的示例，也是我们在试着解决的任务

* 性能和功能分析：内存占用，cpu，time





### Rust Web应用程序简介：https://erwabook.com/intro/

架构:数据库层    ---    数据库访问层（相比于直接修改数据库啥的会更安全） ---  REST API层（基于Rocket构建，这是一个Rust网络框架。我们的前端客户端将向我们的Rocket程序发送HTTP请求。当Rocket调用我们的代码时，我们将使用数据库访问层来读取或写入数据库。REST API代码会将数据整理成适当的格式（JSON），然后再将其返回给客户端。） ---顶层、前端即提供给用户的Web UI(它在Web浏览器中作为WebAssembly（JavaScript）运行。我们将使用Seed框架将Rust代码编译为WebAssembly应用，然后将其加载到浏览器中)

* 环境安装

* 设置ORM和数据库：*可以*只对数据库进行原始查询，然后将数据临时整理为应用程序将使用的类型，但这很复杂且容易出错，[Diesel](https://diesel.rs/)是Rust的ORM，它为我们提供了一种类型安全的方式来与数据库进行交互；此外，`diesel_cli` tool让我们更好的manage项目

* 编写第一次迁移

  * generate the skeleton: 生成migrations文件夹，其下有up和down.sql两个文件，其中分别含有create和delete  table的代码

* 创建数据库访问层：在Diesel代码周围编写一组精简包装并将其作为模块公开

  主要结构：db文件夹：models.rs,schema.rs,mod.rs

  * 在数据库中插入一条记录
  * Create a Development Tool（主要用来测试之类）：在与db文件夹同级的bin文件夹下创建todo.rs,通过`cargo run --bin todo new 'do the thing'`等命令行命令读取指令并对数据库进行相应操作(用到了db文件夹下写的接口)
  * Querying tasks:查询数据库的接口，也实现在db里面

* Create a REST API Layer

  * 创建bin/backend.rs
  * 使用Rocket来mount "/tasks",在有网址访问请求时，通过backend.rs查询数据库并将数据返回到浏览器
  * Serializing to JSON：将数据以json格式返回到浏览器

  更多？数据库连接池（参见Rocket文档）

* 创建基于浏览器的前端UI:基于seed,通过将Rust代码交叉编译到WebAssembly（wasm），我们将在浏览器中运行它

  * 采取 backend 和 frontend两个library crate, 然后交互部分写在root crate

  * 安装wasm工具链

  * cargo make：通过在root crate下的Makefile.toml以及frontend下的Makefile.toml实现对前端后端两个crate的一次性编译

    * With all that in place, now just running `cargo make` in the root will give us:
      * backend library and binaries under target/debug
      * browser-loadable web assembly package in frontend/pkg/package_bg.wasm

  * Behind the scenes:

    * The way our frontend app is going to work:
      * we write some Rust
      * wasm-pack generates some files
        * the .wasm file is a WebAssembly binary
        * the .js file is a JavaScript loader that will pull in the wasm, and it acts as the gatekeeper between JavaScript and Rust
        * package.json has some metadata in case we want to integrate with npm and friends
      * we write an html stub file, that loads the .js, which loads the .wasm
      * our app attaches itself to a DOM element in the html file
      * the browser shows our app's UI elements
      * our users rejoice

  * Create a Stub App

    Let's walk through this starting from the bottom. Everything kicks off with our `render` function because we added the `start` attribute to the `#[wasm_bindgen]` macro. This sets things up so that our function is called as soon as the module is loaded.

    This function creates a seed app, passing in our init, update, and view functions, and then launches the app.

    我们的init函数首先被调用，它负责使用应用程序从其开始的url路径进行任何操作（我们在此不进行处理-本指南中根本不处理任何路由）。然后，它需要创建并返回一个模型，该模型将存储应用程序的状态

    our view function takes the model and returns a DOM node. Here we're simply matching on coming or going and setting an appropriate greeting in our  <h1>. Seed provides macros for all valid HTML5 tags, and as you can see in the xample it also has macros for things like class and style.

    您还可以在这里看到我们如何附加了一个简单的事件处理程序：只要在h1上发生单击（由于样式，它就是视口的整个大小），它将向我们的update函数发送点击消息。

    ```rust
    fn fetch_drills() -> impl Future<Item = Msg, Error = Msg> {
        Request::new("http://localhost:8000/tasks/").fetch_json_data(Msg::FetchedTasks)
    }
    
    fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
        orders.perform_cmd(fetch_drills());
        Model {
            direction: Direction::Coming,
        }
    }
    ```

    orders provides a mechanism for us to be able to add messages or futures to a queue. We can send multiple messages or futures and they will be performed in the order that we call the functions, with futures being scheduled after the model update.

    fetch_drill()函数：Since we want to fetch our tasks, we create a future using the Requests struct, which is seed's wrapper around the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch). We create a new request for a hard-coded (gasp!) url, and then call its `fetch_json_data` method which returns a Future. This future will create the Msg we provided, which will then get pumped into our `update` function when the request completes (or fails).当request成功或失败，都会把Msg参数传入update函数

    所以还需构造新的Msg：enum Msg {    FetchedTasks(fetch::ResponseDataResult<JsonApiResponse>), }

    另外，涉及到对于backend里内容的调用，但后端引入了甚至无法为wasm构建的依赖项，并且第二个原因实际上是相同的：我们不想被迫建立那些额外的依赖项

    所以考虑在root crate中定义需要在前后端同时用到的结构，并将root crate同时引入前后端的Cargo.toml:   mytodo = { path = ".." }

    ```rust
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Task {//与db文件夹下的Task区分开，这个是为了能把数据传到前端而重建的一个结构体
        pub id: i32,
        pub title: String,
    }
    
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct JsonApiResponse {
        pub data: Vec<Task>,
    }
    ```

    最后完善updata函数里对Msg的处理:If we get an Ok then the fetch has succeeded and we should update the model so we can render the tasks in the DOM.

  * Displaying the Tasks:完善view函数
  * Adding CORS Support in the Backend：**跨域资源共享**（[CORS](https://developer.mozilla.org/en-US/docs/Glossary/CORS)）是一种机制，它使用附加的[HTTP](https://developer.mozilla.org/en-US/docs/Glossary/HTTP)标头来告诉浏览器以使Web应用程序在一个[来源](https://developer.mozilla.org/en-US/docs/Glossary/origin)运行，并从另一个[来源](https://developer.mozilla.org/en-US/docs/Glossary/origin)访问选定的资源。Web应用程序请求其来源（域，协议或端口）不同的资源时，将执行跨域HTTP请求

  报错：type mismatch resolving 

  ```rust
  for<'r> <for<'s> fn(seed::Url, &'s mut seed::orders::OrdersContainer<Msg, _, _>) -> Model{
      init::<seed::orders::OrdersContainer<Msg, _, _>>
  }
  as std::ops::FnOnce<(seed::Url, &'r mut seed::orders::OrdersContainer<Msg, _, _>)>>::Output == seed::prelude::Init<_>
  ```

  

  for<'r> <for<'s> fn(seed::Url, &'s mut seed::orders::OrdersContainer<Msg, _, _>) -> Model {init::<seed::orders::OrdersContainer<Msg, _, _>>} as std::ops::FnOnce<(seed::Url, &'r mut seed::orders::OrdersContainer<Msg, _, _>)>>::Output == seed::prelude::Init<_>`





### Seed教程：

#### APP1:Counter

**Model**:你的app的state，多为结构体；尽可能把Model简单化，也不要为Model写任何方法；Model中元素为自定义类型时，自定义类型定义写在Model定义的下方

**Init**:在Web程序运行开始时执行，一般用于返回一个Model;参数一般为url: Url, orders: &mut impl Orders<Msg>，后者有很多方法，可用于giving orders

**Msg**: 在点击按钮等动作发生时会发送Msg;其本身一般为enum，并且是static，即其基本同Model；两种类型的messages：

- Commands - e.g. `ScrollToTop`, `ToggleMenu`, `RemoveItem(ItemId)`, etc.
- Events - e.g. `ButtonClicked`, `UrlChanged(subs::UrlChanged)`, `TextUpdated(String)`, etc.
- Attribute `derive`：如Copy,Clone,Eq,PartialEq

**Update**:收到新的Msg时会调用此函数，用于对Msg作匹配，来执行不同操作,一般是更新Model；如何写好：只要一个match语句，对Option，Result等类型，采用多个匹配（即Ok和Err分为两个元素来作匹配），注释要写好

**View**：用于将Model转化为html；如何写好:学会拆分，根View和嵌套view函数

​	fn view(model: &Model) -> Node<Msg>： model为不可变；Node为HTML元素

​	一般流程：

* init函数执行后会立刻执行一次view以初始化app
* action happens(如button)
* 执行update函数
* 调用view函数
* 页面重新渲染

**Element Macros**: 

```
div![
    C!["counter"],
    "This is a counter.",
]
```

等价于

```
<div class="counter">
    This is a counter.
</div>
```

可以放在其中的：attributes, event handlers, nodes and DOM references；Rust类型：strings and numbers，以及Option,Vec,Iterators(其所含一定也是实现了UpdateEl trait)

```
div![
    IF!(menu.is_visible() => view_menu())
]
```

```
div![
    raw!("<h1>Title</h1>"),
    // Inline `content.html` during compilation.
    raw!(include_str!("../content.html")),  
]
```

**Attributes**

```
div![
    C!["counter", IF!(selected => "active")],
    style!{
        St::Display => "flex",
        St::Padding => px(10),
    },
    attrs!{At::Title => "A Title"},
    "This is a counter.",
]
```

即

```
<div class="counter active" title="A Title" style="display:flex;padding:10px">
    This is a counter.
</div>
```

* C!: items需实现ToClasses trait: String` and `&str`, references and containers `Option` and `Vec；多次C!的使用可以合并：

  ```
  let selected = false;
  let optional_classes: Option<Vec<String>> = None;
  div![
      C!["counter", IF!(selected => "active")],
      C![IF!(true => vec!["class_a", "class_b"])],
      C![optional_classes],
  ]   
  ```

  即<div class="counter class_a class_b"></div>
  
* style!: 是键值对；Key: St::Display或者自定义名字e.g.`St::from("custom_name")`

  Value:实现了ToString trait的，可以使Option

  ```
  let selected = true;
  let apply_custom = true;
  div![
      style!{
          St::Margin => px(50),
          St::MaxWidth => unit!(50, %),
          St::Top => 0,
          St::Padding => px(20) + " " + &px(15)
          St::BackgroundColor => if selected { "green" } else { "white" },
          St::from("custom_name") => IF!(apply_custom => "a_value"),
      }
  ]   
  ```

  等价于

  ```
  <div style="
      margin:50px;
      max-width:50%;
      top:0;
      padding:20px 15px;
      background-color:green;
      custom_name:a_value
  "></div>
  ```

* attrs!:同样是键值对

  Key:e.g. `At::Title`或自定义e.g. `At::from("custom_name")`

  Value:实现了ToString trait：有三类值：Ignored，None，`Some(String)` - If `v` in `At::X => v`,implements `ToString`, then it's automatically transformed to `AtValue::Some(v)`.

  事实上C!和style!只是attrs!的特例，如下所示，不过不建议这样写

  ```
  attrs!{At::Class => "class_a", At::Style => "top:0"}
  ```

  ```
  let disabled = false;
  ...
  attrs!{
      At::Disabled => disabled.as_at_value()
  }
  ```

  等价于

  ```
  attrs!{
      At::Disabled => if disabled { At::None } else { At::Ignored }
  }
  ```

  其他例子：

  ```
  let disabled = true;
  div![
      attrs! {
          At::Disabled => disabled.as_at_value(),
          At::Title => "a_title",
          At::AutoFocus => AtValue::None,
          At::from("custom_name") => 123,
      }
  ]   
  ```

  等价于<div disabled="" title="a_title" autofocus="" custom_name="123"></div>

**Event Handlers:**其原理看的不是很懂

```Rust
button![
    model, 
    ev(Ev::Click, |_| Msg::Increment),//参数：Event（Ev::Click）和Callback（|_| Msg::Increment）
]//点击button，会调用update函数，并以Msg::Increment作为参数
```

**Start:**

1. It mounts the app into the chosen root element. (We'll talk about it more later).
2. Does some low-level app initialization - setups listeners, loads base path for routing, enable panic logging to the console, etc.
3. Calls your `init` function.
4. Render the app for the first time.
5. Returns the `App` instance.It's useful when you need to setup some callbacks as soon as possible (see example [update_from_js](https://github.com/seed-rs/seed/blob/2b134d1de2a8b9aa520d11be6e45eef1e5fcd527/examples/update_from_js/src/lib.rs#L77-L79)).



#### APP 2:TodoMVC

设计思想: 先Model,Msg结构，再其他

* No todos ; 
* New todo;
* Mark all as complete; Item(包括把一个todo切换为completed，edit todo，remove todo三种交互)；
* Editing(编辑模式下对SelectedTodo项的修改);Clear completed button;
* Persistence(使用localStorage来persist data：If the framework has capabilities for persisting data (e.g. Backbone.sync), use that. Otherwise, use vanilla localStorage.)；
* Routing：包含all，active，completed三种状态，当选定一个状态时（此时网页url也会改变），会filter todos，隐藏不需要显示的todos，而且会动态更新(即输入新的todo，那么all和active里均会立即出现)

**Model设计**：Model结构体是对整个页面元素的设计；Todo结构体则是对一个待办事项元素的设计

```rust
struct Model {
    base_url: Url,
    todos: BTreeMap<Ulid, Todo>,
    new_todo_title: String,
    selected_todo: Option<SelectedTodo>,
    filter: Filter,
}

#[derive(Deserialize, Serialize)]
struct Todo {
    id: Ulid,
    title: String,
    completed: bool,
}

struct SelectedTodo {
    id: Ulid,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

// ------ Filter ------

#[derive(Copy, Clone, Eq, PartialEq, EnumIter)]
enum Filter {
    All,
    Active,
    Completed,
}
```



**Msg设计**:对于设计好的Model，根据项目的功能需求，设置相应的Msg项

```rust
enum Msg {
   UrlChanged(subs::UrlChanged),
   NewTodoTitleChanged(String),

   // ------ Basic Todo operations ------

   CreateTodo,
   ToggleTodo(Ulid),
   RemoveTodo(Ulid),
   
   // ------ Bulk operations ------

   CheckOrUncheckAll,
   ClearCompleted,
   
   // ------ Selection ------

   SelectTodo(Option<Ulid>),
   SelectedTodoTitleChanged(String),
   SaveSelectedTodo,
}
```

**Project Setup:**设计好了Model和Msg就可以creating project了：update和view应该是一边写一边验证，快速找问题并解决

**View函数实现**:把大的view函数拆分成几个部分(而且实际上就是用seed宏来替代html文件罢了)，此外就是需要注意具体实现了，比如view返回值为Vec<Node<Msg>> 还是Node<Msg>，以及其内部实现的写法，可通过seed crate查询其宏，大部分能直接与html对应上；

​	此外就是Model的值也是会在这里用到，其数据就是我们需要呈现在前端的相关信息；

However it causes compilation errors because the root `vec![...]` expects only `Node` as items but our `IF!` returns `Option>>`. Fortunately, there is macro `nodes!` that aligns all types to make the compiler happy:

建议:better to pass `Option<&Item>` instead of `&Option`;   Use `.unwrap()` (instead of `expect("...")`) in places when you are SURE the code will work

主要疑问:1. filter那一块的实现；2.新输入的todo项的更新



**Update函数的实现**:主要就是匹配Msg然后做出不同的修改，此外引发事件的按钮（button,keyenter，Ev::Blur）等需要在view函数里相应实现

`orders.after_next_render` registers a callback that is invoked after the next `view` invocation

`input_element.get()` returns `Option` where `E` is a specific DOM element reference like `web_sys::HtmlInputElement`. It returns `None` when the element doesn't exists in the DOM or has an incompatible type => all [ElRef](https://github.com/seed-rs/seed/blob/0a538f03d6aeb56b00d997c80a666e388279a727/src/virtual_dom/el_ref.rs) methods are safe to use.





**LocalStorage**

We need a new dependency [serde](https://crates.io/crates/serde) to **ser**ialize and **de**serialize todos to/from JSON because we can store only JSON strings in `LocalStorage`.

这个类似于在本地存储todos，在同一浏览器不同页面加载该网页时，数据仍在



**Routing**:

* view函数里需根据filter值的不同而显示不同的todos等

* url

  ```rust
  let filter = match url.next_hash_path_part(){
          Some("active") => Filter::Active,
          Some("completed") => Filter::Completed,
          _ => Filter::All,
      };
  ```

  when you call `url.next_hash_path_part()`:

  1. The *path part* (item in `hash_path`) at the position `next_hash_path_part_index` is returned (or `None`)
  2. `next_hash_path_part_index` is incremented.

* `remaining_hash_path_parts()` returns `vec!["active", "foo", "bar"]` for url `/#/active/foo/bar` if you haven't called `url.next_hash_path_part()` before (i.e. if "iterator" starts from 0).

* Subscriptions：Keep in mind there is no magic - `subscribe`, `stream`, `notify`, `UrlChanged`, etc. work the same - Seed or user creates a *notification* (basically any item) and Seed's or user's *subscriptions* (closures) handle it.
* Handle UrlChanged:



**Link Building**:

* const path parts

* Standard link building

  ```rust
  // ----------------- A) -----------------
  
  fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
      ...
      Model {
          base_url: url.to_hash_base_url(),
          ...
          filter: Filter::from(url),
      }
  }
  ```

  `Url` method `to_hash_base_url()` deletes all path parts with index >= `next_hash_path_part_index` in the cloned url. In our case it removes all path parts because `next_hash_path_part_index` is always set to 0 in `url` in `init`.

  ```rust
  // ----------------- B) -----------------
  
  struct Model {
      base_url: Url,
      ...
      filter: Filter,
  }
  ```

  ```rust
  // ----------------- C) -----------------
  
  // ------ ------
  //     Urls
  // ------ ------
  
  struct_urls!();
  impl<'a> Urls<'a> {
      pub fn home(self) -> Url {
          self.base_url()
      }
      pub fn active(self) -> Url {
          self.base_url().add_hash_path_part(ACTIVE)
      }
      pub fn completed(self) -> Url {
          self.base_url().add_hash_path_part(COMPLETED)
      }
  }
  ```

  ```rust
  // ----------------- D) -----------------
  
  fn view(model: &Model) -> Vec<Node<Msg>> {
      ...
              view_footer(&model.todos, model.filter, &model.base_url),
  ...
  
  fn view_footer(todos: &BTreeMap<Ulid, Todo>, selected_filter: Filter, base_url: &Url) -> Node<Msg> {
      ...
          view_filters(selected_filter, base_url),
  ```

  ```rust
  // ----------------- E) -----------------
  
  fn view_filters(selected_filter: Filter, base_url: &Url) -> Node<Msg> {
      ul![C!["filters"],
          Filter::iter().map(|filter| {
              let urls = Urls::new(base_url);
  
              let (url, title) = match filter {
                  Filter::All => (urls.home(), "All"),
                  Filter::Active => (urls.active(), "Active"),
                  Filter::Completed => (urls.completed(), "Completed"),
              };
  
              li![
                  a![C![IF!(filter == selected_filter => "selected")],
                      attrs!{At::Href => url},
                      title,
                  ],
              ]
  .
  ```

* struct_urls!

Seed app blocks should be in this order:

1. `Init`
2. `Model`
3. `Urls` (optional)
4. `Update`
5. `View`
6. `Start`
7. `Exported` (optional, Rust functions available in JS/TS)
8. `Extern` (optional, JS items used in Rust)







数据库的migration操作？

自己写数据库？

### 数据库

本教程约定：SQL关键字总是大写，以示突出，表名和列名均使用小写

#### 关系数据库概述

* 数据库按照数据结构来组织、存储和管理数据，实际上，数据库一共有三种模型：
  * 层次模型
  * 网状模型
  * 关系模型

* 支持的数据类型

* SQL是结构化查询语言的缩写，用来访问和操作数据库系统。SQL语句既可以查询数据库中的数据，也可以添加、更新和删除数据库中的数据，还可以对数据库进行管理和维护操作

  * DDL：Data Definition Language

    DDL允许用户定义数据，也就是创建表、删除表、修改表结构这些操作。通常，DDL由数据库管理员执行。

  * DML：Data Manipulation Language

    DML为用户提供添加、删除、更新数据的能力，这些是应用程序对数据库的日常操作。

  * DQL：Data Query Language

    DQL允许用户查询数据，这也是通常最频繁的数据库日常操作

#### 关系模型