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

* 

# CSS

# DOM

* DOM 是 W3C（万维网联盟）的标准。

  DOM 定义了访问 HTML 和 XML 文档的标准：

  > “W3C 文档对象模型 （DOM） 是中立于平台和语言的接口，它允许程序和脚本动态地访问和更新文档的内容、结构和样式。”

  W3C DOM 标准被分为 3 个不同的部分：

  - 核心 DOM - 针对任何结构化文档的标准模型
  - XML DOM - 针对 XML 文档的标准模型
  - HTML DOM - 针对 HTML 文档的标准模型



