## 浏览器

### 一、js,webassembly,rust结合点

- 创新点：基于wasm可以实现更理想的封装，整体性好，通用性和兼容性强

- 挑战：

  - 利用浏览器进行本地文件的上传下载，前端的搭建

  - 服务器和存储节点

    存储节点是否也是基于浏览器？由于浏览器和存储节点间的编解码和中间传输过程必须经过文件服务器，故文件服务器将是一个瓶颈

    编解码和文件分割/合并的功能前置在浏览器前端，如何实现？

- 发展趋势：跨浏览器的平台通用性，希望能在浏览器端做到和 java 一样的跨平台

### 二、文件上传与下载原理（整合孙一鸣部分+概念补充）

#### 概念补充

*PS: 概念补充是给大家看的，主要解释一些名词*

- HTML标签

  [超文本标记语言](https://baike.baidu.com/item/超文本标记语言/6972570)（HTML）标记标签通常被称为HTML标签，是HTML语言中最基本的单位。就是尖括号括起的关键词，eg.<html><img src="百度百科.jpg" />

- XMLHTTP

  **XMLHTTP是一组API函数集**，可被JavaScript、JScript、VBScript以及其它web浏览器内嵌的脚本语言调用，通过HTTP在浏览器和web服务器之间收发XML或其它数据。XMLHTTP最大的好处在于可以**动态地更新网页**，它无需重新从服务器读取整个网页，也不需要安装额外的插件。该技术被许多网站使用，以实现快速响应的动态网页应用。例如：Google的 Gmail 服务、Google Suggest动态查找界面以及Google Map地理信息服务。

- Ajax （对网页的某部分更新）

  Ajax 即“**A***synchronous* **J***avascript **A**nd* **X***ML*”（异步 JavaScript 和 XML），是指一种创建交互式、快速动态网页应用的网页开发技术，无需重新加载整个网页的情况下，能够更新部分网页的技术。

  通过在后台与服务器进行少量数据交换，Ajax 可以使网页实现异步更新。这意味着可以在不重新加载整个网页的情况下，对**网页的某部分进行更新**。

  Ajax知识体系大梳理：http://louiszhai.github.io/2016/11/02/ajax/ （非常详细）

- Ajax 和 XMLHttpRequest关系

   **AJAX 的要点是 XMLHttpRequest 对象**。不同的浏览器创建 XMLHttpRequest 对象的方法是有差异的。IE 浏览器使用 ActiveXObject，而其他的浏览器使用名为 XMLHttpRequest 的 JavaScript 内建对象。Ajax的核心是JavaScript对象XmlHttpRequest。该对象在Internet Explorer 5中首次引入，它是一种支持异步请求的技术。简而言之，XmlHttpRequest使您可以使用JavaScript向服务器提出请求并处理响应，而不阻塞用户。
  原文链接：https://blog.csdn.net/string_user_get_set/article/details/60467458

- iframe（一种HTML标签）

  IFRAME是HTML标签，作用是文档中的文档，或者浮动的框架(FRAME)。iframe元素会创建包含另外一个文档的内联框架（即行内框架）

- JSON

  JSON（Object Notation, **JS 对象简谱**) 是一种**轻量级的数据交换格式**。它基于 ECMAScript(欧洲计算机协会制定的js规范)的一个子集，采用完全独立于编程语言的文本格式来存储和表示数据。简洁和清晰的层次结构使得 JSON 成为理想的数据交换语言。 易于人阅读和编写，同时也易于机器解析和生成，并有效地提升网络传输效率。

- JSONP

  JSONP(JSON with Padding)是JSON的一种“使用模式”，可**用于解决主流浏览器的跨域数据访问的问题**。由于同源策略，一般来说位于 server1.example.com 的网页无法与不是 server1.example.com的服务器沟通，而 HTML 的<script> 元素是一个例外。利用 <script> 元素的这个开放策略，网页可以得到从其他来源动态产生的 JSON 资料，而这种使用模式就是所谓的 JSONP。用 JSONP 抓到的资料并不是 JSON，而是任意的JavaScript，用 JavaScript 直译器执行而不是用 JSON 解析器解析。

#### 上传原理

**核心：FileUpload对象**

网页上传文件核心：HTML DOM的FileUpload对象

```javascript
<input type="file">
```

在 HTML 文档中该标签每出现一次，一个 FileUpload 对象就会被创建。该标签包含一个按钮，用来打开文件选择对话框，以及一段文字显示选中的文件名或提示没有文件被选中。

把这个标签放在``标签内，设置form的action为服务器目标上传地址，并点击submit按钮或通过JS调用form的submit()方法就可以实现最简单的文件上传了。

```javascript
<form id="uploadForm" method="POST" action="upload" enctype="multipart/form-data">
      <input type="file" id="myFile" name="file"></input>
      <input type="submit" value="提交"></input>
 </form>
```

问题：上传同步、上传完成页面会刷新（所以没人直接这么用，才有了下面的两种）

**XMLHttpRequest Level 2** **/ Ajax**（高版本浏览器）

功能：通过在后台与服务器进行少量数据交换，AJAX 可以使网页实现**异步更新**。这意味着可以在不重新加载整个网页的情况下，对网页的某部分进行更新。

链接：Ajax工作原理 https://blog.csdn.net/weixin_37580235/article/details/81459282 （讲得比较清楚）

1. Ajax所包含技术

   - 使用CSS和XHTML来表示。

   - 使用DOM模型来交互和动态显示。

   - 使用XMLHttpRequest来和服务器进行异步通信。

   - 使用javascript来绑定和调用。
   - 核心是XMLHttpRequest对象（不同浏览器创建的方法有差异）

2. Ajax工作原理

   Ajax的工作原理相当于在用户和服务器之间加了—个中间层(AJAX引擎)，使用户操作与服务器响应异步化。并不是所有的用户请求都提交给服务器。像—些数据验证和数据处理等都交给Ajax引擎自己来做,，只有<u>确定需要从服务器读取新数据时再由Ajax引擎代为向服务器提交请求</u>。

![](https://img-blog.csdn.net/20150716193059952)

上图是Ajax和传统方式的对比图，可以看到Ajax相当于一个中间层。

下面两图对比了交互方式的不同：

- 浏览器的普通交互方式

![](https://img-blog.csdn.net/20150716193857795)

- 浏览器的Ajax交互方式

  ![](https://img-blog.csdn.net/20150716193904120)

  从这两张图可以看出，Ajax使用户操作与服务器响应异步化。

  ![](https://img-blog.csdn.net/20150716193102944)

**iframe + form （较低版本浏览器）**

低版本的IE里的XMLHttpRequest是Level 1，所以不能通过XHR异步向服务器发上传请求，故只能用form的submit。跨域问题借用JSONP

主流浏览器都支持 Ajax ，故对此不作详细介绍

#### 下载原理（孙一鸣的调研）

- 原生提交，后端返回文件流：利用form.submit直接向后端提交,后端返回文件流生成的文件，后端处理成功后会直接返回到页面，浏览器会整理并打开下载文件 。
  优点 ：没有兼容问题，传统方式
  缺点：无法得到后端处理这个过程的时机，无法根据回调函数做交互以及进度提示

- ajax提交，后端返回在线文件地址：利用ajax或者新生的axios去提交请求，后端会返回一个线上的文件地址，前端可以通过原生的window.open打开这个地址就可以实现下载；也可以通过a标签设置href以及download属性，并自动点击实现其下载功能，关于其兼容性问题，可以判断download属性是否存在来弥补。
  优点 ：可以拿到其返回时机，可以做交互
  缺点 ：线上会存储大量的中间临时文件，可以用设置时限来优化。另外涉及用户隐私的问题，可以用token等验证机制实现。

- 前端利用download模块进行下载：其对应的下载文件方案包括了以下几种。

  window.open(url)打开某个文件地址
  iframe的框架中，设置src属性，通过iframe进行文件的下载，支持文件地址
  通过form标签，设置action的文件地址，然后通过form的提交来完成文件的下载（支持二进制）

  对于常规的支持文件地址的下载，兼容性非常好，而对于传统的文件流性质的，通过form标签也可以进行简单的支持。

### 三、V8内核，Node.js原理及对webassembly，js,rust的支持

#### **V8 引擎工作原理** （编译器、线程等）

英文链接：https://www.quora.com/How-does-the-Google-V8-engine-work

翻译链接：https://juejin.im/post/5b5014565188251ad06b6091

#### **Node.js 工作原理**

Node.js 关键词：事件驱动、非阻塞式 I/O、适合在分布式设备上运行数据密集型的实时应用

链接：https://www.cnblogs.com/bingooo/p/6720540.html

**1. 基础架构**

![](http://git.cn-hangzhou.oss.aliyun-inc.com/uploads/beidou/beidou/053df3b1fa045a3d5f6218095d108dff/image.png)



上图是 Node.js 的内部结构图。自底向上主要分成三层：最底层是 Node.js 依赖的各种库，有 V8、libuv 等；中间层是各种 Binding，也就是胶水代码；最上层是应用代码，可使用 Node.js 的各种 API。

- 最底层：Node.js 依赖的库

    - [V8](https://developers.google.com/v8/)
      Google 开源的高性能 JavaScript 引擎，它将 JavaScript 代码转换成机器码，然后执行，因此速度非常快。V8 以 C++ 语言开发，Google 的 Chrome 浏览器正是使用的 V8 引擎。
    - [libuv](https://github.com/libuv/libuv)
      libuv 以 C 语言开发，内部管理着一个线程池。在此基础之上，提供事件循环（Event Loop）、异步网络 I/O、文件系统 I/O等能力。
    - [其他底层依赖库](https://nodejs.org/en/docs/meta/topics/dependencies/)
      如 [c-ares](http://c-ares.haxx.se/)、[crypto (OpenSSL)](https://www.openssl.org/)、[http-parser](https://github.com/nodejs/http-parser) 以及 [zlib](http://zlib.net/)。这些依赖提供了对系统底层功能的访问，包括网络、压缩、加密等。

- 中间层 Binding：桥接作用

  Node.js 底层的依赖库，以 C/C++等多种不同 语言开发，如何让应用代码（JavaScript）能够与这些底层库相互调用呢？这就需要中间层的 Binding 来完成。Binding 是一些胶水代码，能够把不同语言绑定在一起使其能够互相沟通。在 Node.js 中，binding 所做的就是把 Node.js 那些用 C/C++ 写的库接口暴露给 JS 环境。

  中间层中，除了 Binding，还有 Addon。Binding 仅桥接 Node.js 核心库的一些依赖，如果你想在应用程序中包含其他第三方或者你自己的 C/C++ 库的话，需要自己完成这部分胶水代码。你写的这部分胶水代码就称为 Addon。本质上都是完成桥接的作用，使得应用与底层库能够互通有无。

- 最上层：应用层

    开发的应用、npm 安装的包等都运行在这里。

2. **事件循环**（event loop）
 - Node.js 工作流程：

![](http://git.cn-hangzhou.oss.aliyun-inc.com/uploads/beidou/beidou/c55b2e27cbf21eb8e61e0cfe068b33c2/QRePV.jpg)

一个 Node.js 应用启动时，V8 引擎会执行你写的应用代码，保持一份观察者（注册在事件上的回调函数）列表。当事件发生时，它的回调函数会被加进一个事件队列。只要这个队列还有等待执行的回调函数，事件循环就会持续把回调函数从队列中拿出并执行。

在回调函数执行过程中，所有的 I/O 请求都会转发给工作线程处理。libuv 维持着一个线程池，包含四个工作线程（默认值，可配置）。文件系统 I/O 请求和 DNS 相关请求都会放进这个线程池处理；其他的请求，如网络、平台特性相关的请求会分发给相应的系统处理单元进行处理。

安排给线程池的这些 I/O 操作由 Node.js 的底层库执行，完成之后触发相应事件，对应的事件回调函数会被放入事件队列，等待执行后续操作。这就是一个事件在 Node.js 中执行的整个生命周期。

- 事件循环处理过程：

详细内容请参考[Node.js 官方说明](https://nodejs.org/en/docs/guides/event-loop-timers-and-nexttick/)。

一次事件循环，大概可以分为如下几个阶段：

![](http://git.cn-hangzhou.oss.aliyun-inc.com/uploads/beidou/beidou/ec7fb10dec763dc663dbaf80956a2a46/image.png)

> 图中每一个方块，在事件循环中被称为一个阶段(phase)。

每个阶段都有自己独有的一个用于执行回调函数的 FIFO 队列。当事件循环进入一个指定阶段时，会执行队列中的回调函数，当队列中已经被清空或者执行的回调函数个数达到系统最大限制时，事件循环会进入下一个阶段。

上图中总共有6个阶段：

- timers: 该阶段执行由 `setTimeout()` 和 `setInterval()` 设置的回调函数。
- I/O callbacks: 执行除了close 回调、timers 以及
  `setImmediate()` 设置的回调以外的几乎所有的回调。
- idle,prepare: 仅供内部使用。
- poll: 检索新的 I/O 事件；在适当的时候 Node.js 会阻塞等待。
- check: 执行 `setImmediate()` 设置的回调。
- close callbacks: 执行关闭回调。比如： `socket.on('close', ...)`.

这里有个令人困惑的地方，`I/O callbacks` 与 `poll` 这两个阶段有什么区别？ 既然 `I/O callbacks` 中已经把回调都执行完了，还要 `poll` 做什么？

查阅了[libuv 的文档](http://docs.libuv.org/en/v1.x/design.html#the-i-o-loop)后发现，在 libuv 的 event loop 中，`I/O callbacks` 阶段会执行 `Pending callbacks`。绝大多数情况下，在 `poll` 阶段，所有的 I/O 回调都已经被执行。但是，在某些情况下，有一些回调会被延迟到下一次循环执行。也就是说，在 `I/O callbacks` 阶段执行的回调函数，是上一次事件循环中被延迟执行的回调函数。

还需要提到的一点是 `process.nextTick()`。`process.nextTick()` 产生的回调函数保存在一个叫做 `nextTickQueue` 的队列中，不在上面任何一个阶段的队列里面。当当前操作完成后，`nextTickQueue` 中的回调函数会立即被执行，不管事件循环处在哪个阶段。也就是说，在 `nextTickQueue` 中的回调函数被执行完毕之前，事件循环不会往前推进。

#### **V8 对 webassembly、Javascript 的功能支持**

链接：https://v8.dev/features （比较杂，没有一张系统的表格）

例：使用WebAssembly SIMD的快速并行应用程序

https://v8.dev/features/simd 中有具体方法

#### Javascript对webassembly的支持

链接：https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly *（PS：比较重要的链接）*

此网页介绍了描述、方法、构造器、示例、规范、浏览器兼容性等等

**`WebAssembly`**JavaScript 对象是所有 [WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly) 相关功能的命名空间。

和大多数全局对象不一样，`WebAssembly`不是一个构造函数（它不是一个函数对象）。它类似于 [`Math`](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Math) 对象或者 [`Intl`](https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Intl) 对象，Math 对象也是一个命名空间对象，用于保存数学常量和函数；Intl则是用于国际化和其他语言相关函数的命名空间对象。

### 四、内核调用及web前端实现原理

***PS: 关于调用浏览器内核实现文件上传、下载，见第二点***

#### web前端开发

**1. 核心技术**：HTML语言、CSS、JavaScript（骨、肉、魂）

链接：知乎对web前端开发的简单介绍 https://www.zhihu.com/question/28589914/answer/583449419

- 先说“骨”——HTML。HTML,翻译过来就是超文本标记语言，而不是江湖上的HOW TO ML。方向不能搞错了，我们整的东西可是老少咸宜的。HTML学习最重要的标签的学习，div、h1-h6、p、ul-li、strong、图片、字体等，**什么内容用什么框**.

- 再说“肉”——CSS。CSS定义了HTML标签的**显示外观，气质**。主要掌握浮动，宽高设置、显示属性等

- 最后“魂”——Javascript。这是**运行在浏览器上的脚本**，但是现在javascript已经远远不是当年的那个js了，尤其Ecmascript6标准出来后，nodeJS 横空出世，JS暴露出一统天下的野心，JS让网页变得灵活，其实现的每一个明里暗里的**交互**，其实是为了触及您的灵魂，这也是其成为魂的原因。

**2.环境基础设备、浏览器以及工作原理**

必须指出的是，**html CSS JS都是运行在浏览器的，是由浏览器负责编译和呈现的**。所以必须了解浏览器的工作原理。但是浏览器千千万万，也不是每个都要去解剖，主要的有Chrome, Firefox, IE，Safari,Opera,国内的主浏浏览器基本是基于chrome内核开发，做了一些更为接地气的功能，了解下就可以了，主要有QQ浏览器，UC，百度浏览器，360浏览器，搜狗浏览器，猎豹浏览器等。

**3 计算机基础**

计算机网络，http协议。既然是web必不可少需要知道计算机网络的知识，这对于网页的加载和速度优化有很大的帮助，并且，我们做的不是静态的页面，而是动态的，所以必然涉及到与后台之间的数据的传输和存储，这个是要掌握的。必须懂：Ajax，必须会的工具：fiddler

**4. web前端流行框架**

Bootstrap、jQuery UI、Amaze UI、jquery mobile、angular、Vue.、React ……

*PS：浏览器的工作原理在之前的调研报告里有*

### 五、找到的一些其他资料

#### **Blazor：C#**  （ps：做个引入）

Blazor 是一个 Web UI 框架，可通过 WebAssembly 在任意浏览器中运行 .Net 。也就说，你可以用C# 写前端。可以理解为，这是一个C#语言的Vue, Angular, React（ps: web前端的几种框架）。

#### **Yew：Rust**  （重点）

链接：Yew官方文档 https://yew.rs/docs/v/zh_cn/

​			github：https://github.com/jetli/awesome-yew

简介：

**Yew** 是一个设计先进的 [Rust](https://www.rust-lang.org/) 框架，目的是使用 [WebAssembly](https://webassembly.org/) 来创建多线程的前端 web 应用。

- **基于组件的框架**，可以轻松的创建交互式 UI。拥有 [React](https://reactjs.org/) 或 [Elm](https://elm-lang.org/) 等框架经验的开发人员在使用 Yew 时会感到得心应手。
- **高性能** ，前端开发者可以轻易的将工作分流至后端来减少 DOM API 的调用，从而达到异常出色的性能。
- **支持与 Javascript 交互** ，允许开发者使用 NPM 包，并与现有的 Javascript 应用程序结合。

知乎上一个Yew + Rust 的尝试经历：https://zhuanlan.zhihu.com/p/101118828