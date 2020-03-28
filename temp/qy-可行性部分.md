# 1 Node.js 与 WebAssembly

## 1.1 Node.js 简介

* Node.js 是一个基于Chrome JavaScript 运行时建立的一个平台， 用于方便地搭建响应速度快、易于扩展的网络应用。

  Node.js 使用事件驱动， 非阻塞 I/O 模型而得以轻量和高效，非常适合在分布式设备上运行数据密集型的实时应用。

* Node.js 让 JS 可以开发后端程序，实现几乎其他后端语言实现的所有功能，可以与 PHP、Java、Python、.NET、Ruby 等后端语言平起平坐。

* Node.js 作为 JS 的运行环境运行在服务器端，作为 web server 运行在本地，用作打包工具或者构建工具。具体地可以应用于搭建分布式服务器框架，物联网开发框架，即使通讯，爬虫，开发动态网站，为各类网络应用提供 API 接口等。


## 1.2 使用 Node.js 运行 WebAssembly 文件的方法：

* ~~通过开启 --expose-wasm 参数，在 Node.JS 可以访问全局对象 Wasm , 通过它可以创建 WebAssembly 模块。~~

  ~~(我尝试的时候报错：expose is not defined,可能是因为node.js版本差异，教程中为7.2.1，我下载版本为12.16.1)（另一个ubuntu系统中8.10.0版本的node.js可以开启参数，但是后续步骤里又报Wasm is not defined）~~

  * 高层语言如 C/C++, Rust, Go 等一般可以通过工具直接编译成 WebAssembly 。在将更高层的语言编译为 WebAssembly 时，一些编译器会输出一个 .js 文件和一个 .wasm 文件， JS 代码加载执行 WebAssembly 。如果代码中只含有执行计算的代码，也可以只编译为一个 .wasm 文件。

  * .wasm 文件编译完成后，需要将 .wasm 文件在 .js 文件中加载到 buffer 区，并将 buffer 转换为 typed array 类型，再实例化 Node.js 中的 WebAssembly 模块，最后使用 node 命令直接执行 .js 文件。

  * ~~~
    //load to buffer
    const fs = require('fs');
    var source = fs.readFileSync('./test.wasm');
    var typedArray = new Uint8Array(source);
    
    //instantiate the WebAssembly module
    const env = {
        memoryBase: 0,
        tableBase: 0,
        memory: new WebAssembly.Memory({
          initial: 256
        }),
        table: new WebAssembly.Table({
          initial: 0,
          element: 'anyfunc'
        })
      }
    
    WebAssembly.instantiate(typedArray, {
      env: env
    }).then(...   //according to specific wasm code
    ).catch(e => {
      // error caught
      console.log(e);
    });
    ~~~

  

# 2 可能与本项目有关的工具和库

### 1.2.1 WebAssembly 相关

* seed

  使用 WebAssembly 构建网页应用的 Rust 框架。

* wasm-bindgen

  提供 WebAssembly 模块与 JS, Rust 交互的支持。使 WebAssembly 可以调用 JS API，也可以使 WebAssembly 中暴露函数给 JS 调用。

### 1.2.2 Rust 相关

* cargo-web

  支持功能：`cargo web build`  使用 Rust 后端构建项目，可以直接交叉编译出 WebAssembly ；`cargo web test`  在 Google Chrome 环境或 Node.js 下运行测试；`cargo web start`  构建项目，并在一个 embeded webserver 上运行。

* iron, rocket, nickel

  三者均为 Rust web 开发的现成框架。以 nickel 的使用为例：
  
  * 在 Cargo.toml 文件的 dependencies 字段下加入: `nickel = "*"`
  
  * 在 main.rs 中导入 nickel ，并加载其中定义的所有宏：
  
    ~~~
    #[macro_use] extern crate nickel;
    use nickel::Nickel;
    ~~~
  
  * 使用 nickel 中定义的服务器类型：
  
    ~~~
    let mut server = Nickel::new();
    ~~~
  
  * 使用 utilize 方法来声明服务器实例的主要功能。



参考网站：

[在Node.JS 中体验 WebAssembly](https://zhuanlan.zhihu.com/p/25619626)

[How to Use WebAssembly with Node.js](https://www.codepool.biz/use-webassembly-node-js.html)

[WebAssembly 官网文档](https://developer.mozilla.org/en-US/docs/WebAssembly/Using_the_JavaScript_API)

[WebAssembly lib](https://lib.rs/wasm)

[cargo-web](https://github.com/koute/cargo-web)

[Rust 开发 Web 应用程序](https://www.codercto.com/a/35023.html)

[Rust 中 nickle 的使用](https://blog.csdn.net/m0_37696990/article/details/82811037)

[Node.js 介绍](https://blog.csdn.net/qq_36742720/java/article/details/83820277)