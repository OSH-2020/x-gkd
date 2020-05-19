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