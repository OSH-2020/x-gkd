

# FragmentManager.rs

* 关于Rust报错的机制与Java的对应有待完善，包括
  * @SuppressWarnings("deprecation")
  * 函数后面+throws Exception
* 关于Rust File和Path中的方法搞得不是很懂，原Java程序中的FragmentFolder和fragmentID分别对应要创建的文件路径和文件名，但是感觉在Rust中应该主要使用Path类，以及文件路径在不同系统上的表示有待进一步研究
* 对于if语句中的is_ok存疑
* 其他小的问题在注释中给出了

# Seed

## auth

* 一个关于登录的页面，有home login和logout

## drop_zone

* 显示一个文件，类似于用记事本的格式打开这个文件

## fetch.

* 通过本地的json文件与页面进行交互

## graphql

* 分层显示大洲、国家等等

## i18N example

* ![image-20200706231648169](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200706231648169.png)

## markdown

* md和HTML的转换

## Page

* 页面转换，在一个app中浏览多个页面

* 三个项目有一定的区别，还未研究

## Server integration example.

* 客户端和服务端的一些操作
* 具有文件上传按钮，**可以浏览本地文件**，有待考虑

## Subscribe

* 没太搞懂，有待研究
* How to create and use subscriptions, streams, notifications and commands.

## tea_component

* 没太搞懂
* How to write a component in The Elm architecture.
* You'll also learn how to pass messages to the parent component.

## Unsaved changes

* 如果网页有未保存的东西，则跳转时会弹出提醒框

## Update from JS.

* How to trigger `update` function from Javascript world.
* You'll also see how to call JS functions from Rust.

## url

* 一些关于url的用法

## Windows_event

* 获得当前鼠标在屏幕上的坐标

# Weightrs下载部分简单分析

## URL与URI

### URL

* URL（统一资源定位符）是Internet上资源的地址，可以定义为引用地址的字符串，用于指示资源的位置以及用于访问它的协议。
* URL是在网络上定位资源的最普遍使用的方式，它提供了一种通过描述其网络位置或主要访问机制来检索物理位置的表示的方法。
* URL中描述了协议，该URL用于检索资源和资源名称。如果资源是Web类型资源，则URL在开头包含http / https。同样，如果资源是文件，则以ftp开头，如果资源是电子邮件地址，则以mailto开头。

* URL包含以下信息
  1. 用于访问资源的协议
  2. 服务器的位置（无论是通过IP地址还是域名）
  3. 服务器上的端口号（可选）
  4. 资源在服务器目录结构中的位置
  5. 片段标识符（可选）

### URI

* URI（统一资源标识符）是标识逻辑或物理资源的字符序列，与URL类似，也是一串字符。通过使用位置，名称或两者来标识Internet上的资源；它允许统一识别资源。

* 有两种类型的URI，统一资源标识符（URL）和统一资源名称（URN）。

  ![img](https://img.php.cn/upload/article/000/000/024/5c0a0c05c80e6777.jpg)

* 但是比较疑惑的是以下函数中URI的格式和标准格式有很大差别

## 函数

```rust
fn download_data_uri_as(data_uri: &str, filename: &str) {
    let element = seed::document()
        .create_element("a")
        .expect("should be able to create element");

    let _ = element.set_attribute("href", data_uri);
    let _ = element.set_attribute("download", filename);

    let event = seed::document()
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()")
        .dyn_into::<web_sys::MouseEvent>()
        .ok()
        .expect("should be a MouseEvent");
    event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg("click", true, true);
    let _ = element.dispatch_event(&event);

    element.remove();
}
```

* 传入参数data_uri为文件的某种属性，filename为下载出来的文件名，格式为`current_date-weight-export.form`

* document():Convenience function to access the `web_sys` DOM document.

  * web_sys:Raw API bindings for Web APIs

  * Document.createElement()

    * 语法：var element = document.createElement(tagName[, options]);

    * 用处：In an [HTML](https://developer.mozilla.org/en-US/docs/Web/HTML) document, the **`document.createElement()`** method creates the HTML element specified by tagName, or an [`HTMLUnknownElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUnknownElement) if tagName isn't recognized.

    * 参数：

      * tagName：A string that specifies the type of element to be created. The [`nodeName`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName) of the created element is initialized with the value of tagName. Don't use qualified names (like "html:a") with this method. When called on an HTML document, `createElement()` converts tagName to lower case before creating the element. In Firefox, Opera, and Chrome, `createElement(null)` works like `createElement("null")`
      * options：

    * 返回值：The new Element

    * Element：https://developer.mozilla.org/en-US/docs/Web/API/Element

    * 实例：This creates a new `<div>` and inserts it before the element with the ID "`div1`".

      HTML：

      ```html
      <!DOCTYPE html>
      <html>
      <head>
        <title>||Working with elements||</title>
      </head>
      <body>
        <div id="div1">The text above has been created dynamically.</div>
      </body>
      </html>
      ```

      JS：

      ```javascript
      document.body.onload = addElement;
      
      function addElement () { 
        // create a new div element 
        var newDiv = document.createElement("div"); 
        // and give it some content 
        var newContent = document.createTextNode("Hi there and greetings!"); 
        // add the text node to the newly created div
        newDiv.appendChild(newContent);  
      
        // add the newly created element and its content into the DOM 
        var currentDiv = document.getElementById("div1"); 
        document.body.insertBefore(newDiv, currentDiv); 
      }
      ```

      ![image-20200708113531108](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200708113531108.png)

  * Document.createEvent()：

    * Event：https://developer.mozilla.org/en-US/docs/Web/API/Event

* element.setAttribute：

  * 语法：Element.setAttribute(name, value);
  * 功能：Sets the value of an attribute on the specified element. If the attribute already exists, the value is updated; otherwise a new attribute is added with the specified name and value.
  * 更多参考：https://developer.mozilla.org/en-US/docs/Web/API/Element/setAttribute

* ```
  event.initMouseEvent(type, canBubble, cancelable, view,
                       detail, screenX, screenY, clientX, clientY,
                       ctrlKey, altKey, shiftKey, metaKey,
                       button, relatedTarget);
  ```

* 整体来看：

  * 先建立(create_element)了`<a>`标签，该标签用来定义超链接，即从一张页面链接到另一张页面
  * 再设置该标签的属性(set_attribute)
    * href:指示链接的目标，值为链接
    * download:规定被超链接下载的目标，值为文件名
  * 接着定义事件（event），建立MouseEvents，和点击有关，具体参考https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent
  * 派发事件，并以合适的顺序**同步调用**目标元素相关的事件处理函数
  * 清除Element

# HTML

## 简介

* HTML 是用来描述网页的一种语言。

  - HTML 指的是超文本标记语言 (**H**yper **T**ext **M**arkup **L**anguage)
  - HTML 不是一种编程语言，而是一种*标记语言* (markup language)
  - 标记语言是一套*标记标签* (markup tag)
  - HTML 使用*标记标签*来描述网页

* ## HTML 标签

  HTML 标记标签通常被称为 HTML 标签 (HTML tag)。

  - HTML 标签是由*尖括号*包围的关键词，比如 <html>
  - HTML 标签通常是*成对出现*的，比如 <b> 和 </b>
  - 标签对中的第一个标签是*开始标签*，第二个标签是*结束标签*
  - 开始和结束标签也被称为*开放标签*和*闭合标签*

* ## HTML 文档 = 网页

  - HTML 文档*描述网页*
  - HTML 文档*包含 HTML 标签*和纯文本
  - HTML 文档也被称为*网页*

  Web 浏览器的作用是读取 HTML 文档，并以网页的形式显示出它们。浏览器不会显示 HTML 标签，而是使用标签来解释页面的内容：

* ```html
  <html>
  <body>
  
  <h1>我的第一个标题</h1>
  
  <p>我的第一个段落。</p>
  
  </body>
  </html>
  ```

  ### 例子解释

  - < html> 与 </html> 之间的文本描述网页
  - < body> 与 </body> 之间的文本是可见的页面内容
  - < h1> 与 </h1> 之间的文本被显示为标题
  - < p > 与 </p> 之间的文本被显示为段落

## 编辑器

## 基础

* HTML 标题（Heading）是通过 <h1> - <h6> 等标签进行定义的。

* HTML 段落是通过 <p> 标签进行定义的。

* HTML 链接是通过 <a> 标签进行定义的。

  ### 实例

  ```html
  <a href="http://www.w3school.com.cn">This is a link</a>
  ```

* HTML 图像是通过 <img> 标签进行定义的。

  ### 实例

  ```html
  <img src="w3school.jpg" width="104" height="142" />
  ```

## 元素（element）

* HTML 元素指的是从开始标签（start tag）到结束标签（end tag）的所有代码。

* ## HTML 元素语法

  - HTML 元素以*开始标签*起始
  - HTML 元素以*结束标签*终止
  - *元素的内容*是开始标签与结束标签之间的内容
  - 某些 HTML 元素具有*空内容（empty content）*
  - 空元素*在开始标签中进行关闭*（以开始标签的结束而结束）
  - 大多数 HTML 元素可拥有*属性*

* ## 嵌套的 HTML 元素

  大多数 HTML 元素可以嵌套（可以包含其他 HTML 元素）。

  HTML 文档由嵌套的 HTML 元素构成。

* ## 空的 HTML 元素

  ```
  没有内容的 HTML 元素被称为空元素。空元素是在开始标签中关闭的。
  
  <br> 就是没有关闭标签的空元素（<br> 标签定义换行）。
  
  在 XHTML、XML 以及未来版本的 HTML 中，所有元素都必须被关闭。
  
  在开始标签中添加斜杠，比如 <br />，是关闭空元素的正确方法，HTML、XHTML 和 XML 都接受这种方式。
  
  即使 <br> 在所有浏览器中都是有效的，但使用 <br /> 其实是更长远的保障。
  ```

* ## HTML 提示：使用小写标签

  HTML 标签对大小写不敏感：<P> 等同于 <p>。许多网站都使用大写的 HTML 标签。

  W3School 使用的是小写标签，因为万维网联盟（W3C）在 HTML 4 中*推荐*使用小写，而在未来 (X)HTML 版本中*强制*使用小写。

## 属性

* ## HTML 属性

  HTML 标签可以拥有*属性*。属性提供了有关 HTML 元素的*更多的信息*。

  属性总是以名称/值对的形式出现，比如：*name="value"*。

  属性总是在 HTML 元素的*开始标签*中规定。

* 实例：标题居中、主体字体颜色、表格边框

* ## 始终为属性值加引号

  属性值应该始终被包括在引号内。双引号是最常用的，不过使用单引号也没有问题。

  在某些个别的情况下，比如属性值本身就含有双引号，那么您必须使用单引号，例如：

  ```html
  name='Bill "HelloWorld" Gates'
  ```

* 下面列出了适用于大多数 HTML 元素的属性：

  | 属性  | 值                 | 描述                                     |
  | :---- | :----------------- | :--------------------------------------- |
  | class | *classname*        | 规定元素的类名（classname）              |
  | id    | *id*               | 规定元素的唯一 id                        |
  | style | *style_definition* | 规定元素的行内样式（inline style）       |
  | title | *text*             | 规定元素的额外信息（可在工具提示中显示） |

* 参考手册：https://www.w3school.com.cn/tags/index.asp

## 标题

* 标题（Heading）是通过 <h1> - <h6> 等标签进行定义的。

```
<h1> 定义最大的标题。<h6> 定义最小的标题。
```

* **注释：**浏览器会自动地在标题的前后添加空行。

  **注释：**默认情况下，HTML 会自动地在块级元素前后添加一个额外的空行，比如段落、标题元素前后。

* ## 标题很重要

  请确保将 HTML heading 标签只用于标题。不要仅仅是为了产生粗体或大号的文本而使用标题。

  搜索引擎使用标题为您的网页的结构和内容编制索引。

  因为用户可以通过标题来快速浏览您的网页，所以用标题来呈现文档结构是很重要的。

* ## HTML 水平线

  <hr /> 标签在 HTML 页面中创建水平线。

  hr 元素可用于分隔内容。

* ## HTML 注释

  可以将注释插入 HTML 代码中，这样可以提高其可读性，使代码更易被人理解。浏览器会忽略注释，也不会显示它们。

  注释是这样写的：

  ### 实例

  ```html
  <!-- This is a comment -->
  ```

## 段落

* 段落是通过 <p> 标签定义的。

* 使用空的段落标记 <p></p> 去插入一个空行是个坏习惯。用 <br /> 标签代替它

* ## HTML 折行

  如果您希望在不产生一个新段落的情况下进行换行（新行），请使用 <br /> 标签：

  ```html
  <p>This is<br />a para<br />graph 
      
      with 
      
      
      
      
      
      
      
      line breaks</p>
  ```

* ## HTML 输出 - 有用的提示

  我们无法确定 HTML 被显示的确切效果。屏幕的大小，以及对窗口的调整都可能导致不同的结果。

  对于 HTML，您无法通过在 HTML 代码中添加额外的空格或换行来改变输出的效果。

  当显示页面时，浏览器会移除*源代码中*多余的空格和空行。所有连续的空格或空行都会被算作一个空格。需要注意的是，HTML 代码中的所有连续的空行（换行）也被显示为一个空格。

## 样式

* ## HTML 的 style 属性

  style 属性的作用：

  **提供了一种改变所有 HTML 元素的样式的通用方法。**

* 通过 HTML 样式，能够通过使用 style 属性直接将样式添加到 HTML 元素，或者间接地在独立的样式表中（CSS 文件）进行定义。

* ## 不赞成使用的标签和属性

  在 HTML 4 中，有若干的标签和属性是被废弃的。被废弃（Deprecated）的意思是在未来版本的 HTML 和 XHTML 中将不支持这些标签和属性。

  这里传达的信息很明确：请避免使用这些被废弃的标签和属性！

  ### 应该避免使用下面这些标签和属性：

  | 标签                 | 描述               |
  | :------------------- | :----------------- |
  | <center>             | 定义居中的内容。   |
  | <font> 和 <basefont> | 定义 HTML 字体。   |
  | <s> 和 <strike>      | 定义删除线文本     |
  | <u>                  | 定义下划线文本     |
  | **属性**             | **描述**           |
  | align                | 定义文本的对齐方式 |
  | bgcolor              | 定义背景颜色       |
  | color                | 定义文本颜色       |

  对于以上这些标签和属性：请使用样式代替！

* ## HTML 样式实例 - 背景颜色

  background-color 属性为元素定义了背景颜色

* ## HTML 样式实例 - 字体、颜色和尺寸

  font-family、color 以及 font-size 属性分别定义元素中文本的字体系列、颜色和字体尺寸

* ## HTML 样式实例 - 文本对齐

  text-align 属性规定了元素中文本的水平对齐方式

## 格式化

## HTML 文本格式化实例

- [文本格式化](https://www.w3school.com.cn/tiy/t.asp?f=html_textformatting)

  此例演示如何在一个 HTML 文件中对文本进行格式化

- [预格式文本](https://www.w3school.com.cn/tiy/t.asp?f=html_preformattedtext)

  此例演示如何使用 pre 标签对空行和空格进行控制。

- [“计算机输出”标签](https://www.w3school.com.cn/tiy/t.asp?f=html_computeroutput)

  此例演示不同的“计算机输出”标签的显示效果。

- [地址](https://www.w3school.com.cn/tiy/t.asp?f=html_address)

  此例演示如何在 HTML 文件中写地址。

- [缩写和首字母缩写](https://www.w3school.com.cn/tiy/t.asp?f=html_abbracronym)

  此例演示如何实现缩写或首字母缩写。

- [文字方向](https://www.w3school.com.cn/tiy/t.asp?f=html_bdo)

  此例演示如何改变文字的方向。

- [块引用](https://www.w3school.com.cn/tiy/t.asp?f=html_quotations)

  此例演示如何实现长短不一的引用语。

- [删除字效果和插入字效果](https://www.w3school.com.cn/tiy/t.asp?f=html_delins)

  此例演示如何标记删除文本和插入文本。

- 更多请参考：https://www.w3school.com.cn/html/html_formatting.asp

## 引用

* ## HTML <q> 用于短的引用

  HTML *<q>* 元素定义*短的引用*。

  浏览器通常会为 <q> 元素包围*引号*。

* ## 用于长引用的 HTML <blockquote>

  HTML *<blockquote>* 元素定义被引用的节。

  浏览器通常会对 <blockquote> 元素进行*缩进*处理。

* ## 用于缩略词的 HTML <abbr>

  HTML *<abbr>* 元素定义*缩写*或首字母缩略语。

  对缩写进行标记能够为浏览器、翻译系统以及搜索引擎提供有用的信息。

* ## 用于定义的 HTML <dfn>

  * 没太看懂https://www.w3school.com.cn/html/html_quotation_elements.asp

* ## 用于联系信息的 HTML <address>

  HTML *<address>* 元素定义文档或文章的联系信息（作者/拥有者）。

  此元素通常以*斜体*显示。大多数浏览器会在此元素前后添加折行。

* ## 用于著作标题的 HTML <cite>

  HTML *<cite>* 元素定义*著作的标题*。

  浏览器通常会以斜体显示 <cite> 元素。

* ## 用于双向重写的 HTML <bdo>

  HTML *<bdo>* 元素定义双流向覆盖（bi-directional override）。

* ## HTML 引文、引用和定义元素

  | 标签         | 描述                             |
  | :----------- | :------------------------------- |
  | <abbr>       | 定义缩写或首字母缩略语。         |
  | <address>    | 定义文档作者或拥有者的联系信息。 |
  | <bdo>        | 定义文本方向。                   |
  | <blockquote> | 定义从其他来源引用的节。         |
  | <dfn>        | 定义项目或缩略词的定义。         |
  | <q>          | 定义短的行内引用。               |
  | <cite>       | 定义著作的标题。                 |

## 计算机代码

* ```html
  var person = {
      firstName:"Bill",
      lastName:"Gates",
      age:50,
      eyeColor:"blue"
  }
  ```

* ## HTML 计算机代码格式

  通常，HTML 使用*可变*的字母尺寸，以及可变的字母间距。

  在显示*计算机代码*示例时，并不需要如此。

  *<kbd>*, *<samp>*, 以及 *<code>* 元素全都支持固定的字母尺寸和间距。

* ## HTML 键盘格式

  HTML *<kbd>* 元素定义*键盘输入*

* ## HTML 样本格式

  HTML *<samp>* 元素定义*计算机输出示例*

* ## HTML 代码格式

  HTML *<code>* 元素定义*编程代码示例*

  <code> 元素不保留多余的空格和折行

  如需解决该问题，必须在 <pre> 元素中包围代码

* ## HTML 变量格式化

  HTML *<var>* 元素定义*数学变量*

* 总结

* ## HTML 计算机代码元素

  | 标签   | 描述               |
  | :----- | :----------------- |
  | <code> | 定义计算机代码文本 |
  | <kbd>  | 定义键盘文本       |
  | <samp> | 定义计算机代码示例 |
  | <var>  | 定义变量           |
  | <pre>  | 定义预格式化文本   |



# CSS

# DOM

* DOM 是 W3C（万维网联盟）的标准。

  DOM 定义了访问 HTML 和 XML 文档的标准：

  > “W3C 文档对象模型 （DOM） 是中立于平台和语言的接口，它允许程序和脚本动态地访问和更新文档的内容、结构和样式。”

  W3C DOM 标准被分为 3 个不同的部分：

  - 核心 DOM - 针对任何结构化文档的标准模型
  - XML DOM - 针对 XML 文档的标准模型
  - HTML DOM - 针对 HTML 文档的标准模型

# 17级web工程部分

## Tomcat与Servlet关系

1. 浏览器产生HTTP请求给Tomcat
2. Tomcat将HTTP解析为request
3. Tomcat从磁盘加载servlet
4. 把request给servlet处理，并返回response
5. Tomcat将response转换为HTTP响应
6. 发回给前端

## struts2框架

![image-20200711200406395](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200711200406395.png)

## 登录注册模块

### index.html(友好的网页)

* 主要采用了bootstrap 主题，采用了其提供的垂直表单元素
* 同时注册和登录表单是可以单击进行切换的，使用了bootstrap 切换卡元素

### index_ajax.js(让网页具有和用户和服务器交互的动态能力)

* 主要是为两个button（注册和登录）提供了对应的js 动作
* 调用异步的ajax 将表单信息提交给服务器并调用下面两个java 程序之一进行服务

1. UserLogin.java------接受来自网页的登录请求，查询数据库进行身份核实和反馈核实结
   果
2. UserReg.java------接收来自网页的用户注册请求，将请求插入数据库等待管理员审核。
   基本和UserLogin 类似

### 网页主界面

#### majorPage.jsp

* 包含了界面的主要的html 代码。之所以还采用了jsp 在服务器端动
  态生成html 代码，是因为第一次打开该网页就会展示文件系统根目录文件夹信息，这些信
  息是动态的。因此我动态的查询数据库并返回html 代码

#### 文件目录展示交互模块

* 用户可以单击进入子目录或者返回上层目录，同时当前访问路径导航栏随之刷新
* GetFileList.java------根据输入查询的全路径；输出该路径下的全部列表项的html 代
  码。
* majorPage_ajax.js------包含该模块主要代码
* 点击列表项之后，首先判断是点击的文件夹------从列表项获取对应的文件夹名称；
  将该文件夹加入全路径；
  刷新导航栏；
  通过ajax 向服务器查询新路径下文件列表。
* 还是文件------控制台显示，点击的是文件，不可进入；不采取其他措施
* 还是返回上一层------ 将全路径最里层元素去掉；
  刷新导航栏；
  通过ajax 向服务器查询新路径下文件列表。

### 文件下载模块

* 该模块提供了用户选中单个文件并进行下载的全套服务。
  用户选中单个文件，点击下载，服务器开始收集碎片，实时反馈进度，网页进度条实时更
  新，进度100%后可单击进度条下载该文件。

* FileDownloader.java------包含了三个功能函数

  * downloadRegister()
    将一条下载请求插入数据库，这样的话服务器将知道要从各个客户端收集该指定的碎片。该函数调用了数据库访问函数包：import database.*

  * progressCheck()
    服务器查询特定本地临时碎片数目，计算出碎片收集进度并且返回给网页。
    该函数调用了本地文件访问接口

  * decodeFile()

    服务器调用erasurecode 开源解码程序，将特定文件复原，等待用户通过http 请求下载

* majorPage_ajax.js------包含该模块的主要代码

  * 当用户点击下载后，遍历列表，对于每一个勾选项进行下载操作。
  * 下载操作：
    * 利用ajax 调用动态方法FileDownloader!downloadRegister，请求服务器收集碎片，为文件任务添加进度条
    * 定时通过ajax 调用动态方法FileDownloader!progressCheck 检测收集进度，并刷新进度条，如果进度到达100%，利用ajax 调用动态方法FileDownloader!decodeFile 进碎片远程拼接，为进度条添加下载属性，链接到生成的要下载的文件

### 异步and同步ajax？

