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





二维数组

