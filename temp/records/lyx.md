### 项目

出发点: 应用WebAssembly和Rust 实现浏览器上运行程序

找到了这样一个可行的项目，即分布式文件系统，那么我们改写出来的系统的特点在什么地方?

* 全打包为Wasm, 兼容性、移植性
* 与js交互(web服务应用)



### Rust总结:

* 多线程

  thread::spawn,thread::sleep;JoinHandle的join方法

  move:移动数据到新线程

  通道 ，send，recv，try_recv

  互斥器(Mutex)

  std::sync::Condvar::wait和std::sync::Condvar::notify_one，std::sync::Condvar::notify_all

  原子类型std::sync::atomic

* unwrap():调用 `option.unwrap()` 来获取 `option` 中包裹的值,出错则直接panic

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





### Rocket

* Lifecycle：Routing, validation,processing,response

  - A set of parameters to match an incoming request against.
  - A handler to process the request and return a response

* 路由(Routing):

  要匹配的参数包括静态路径，动态路径，路径段，表单，查询字符串，请求格式 说明符和主体数据

* 挂载(Mounting):

* Requests:

  * methods
  * dynamic paths
  * Forwarding（含Default ranking)
  * Query Strings
  * Request Guards

