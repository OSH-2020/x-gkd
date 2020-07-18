# Yew_Demo

## 简介

Yew 是一个设计先进的 Rust 框架，目的是使用 WebAssembly 来创建多线程的前端 web 应用。

* 基于组件的框架，可以轻松的创建交互式 UI。拥有 React 或 Elm 等框架经验的开发人员在使用 Yew 时会感到得心应手。

* 高性能 ，前端开发者可以轻易的将工作分流至后端来减少 DOM API 的调用，从而达到异常出色的性能。

* 支持与 Javascript 交互 ，允许开发者使用 NPM 包，并与现有的 Javascript 应用程序结合

## 工具

* **wasm-pack**
* **cargo-web**:一个用来构建 web 客户端应用的 Cargo 子命令，它让构建和部署 web 应用变得非常的简单。它同样也是唯一 一个支持生成 Emscripten 目标代码的工具链。

## 过程

* 创建一个二进制项目

  > cargo new --bin yew-app && cd yew-app

* 在项目中添加yew为依赖库

  ```toml
  [package]
  name = "yew-app"
  version = "0.1.0"
  authors = ["Yew App Developer <name@example.com>"]
  edition = "2018"
  
  [dependencies]
  yew = { version = "0.13.0", features = ["std_web"] }
  ```

* 添加示例代码，功能：构建你的称为 `App` 的 `Component` 根组件，它会显示一个按钮，当你点击它时，`App` 将会更新自己的状态

  ```rust
  use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};
  
  struct App {
      clicked: bool,
      onclick: Callback<ClickEvent>,
  }
  
  enum Msg {
      Click,
  }
  
  impl Component for App {
      type Message = Msg;
      type Properties = ();
  
      fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
          App {
              clicked: false,
              onclick: link.callback(|_| Msg::Click),
          }
      }
  
      fn update(&mut self, msg: Self::Message) -> ShouldRender {
          match msg {
              Msg::Click => {
                  self.clicked = true;
                  true // 指示组件应该重新渲染
              }
          }
      }
  
      fn view(&self) -> Html {
          let button_text = if self.clicked { "Clicked!" } else { "Click me!" };
  
          html! {
              <button onclick=&self.onclick>{ button_text }</button>
          }
      }
  }
  
  fn main() {
      yew::start_app::<App>();
  }
  ```

* 运行

  > cargo web start

  `cargo-web` 将会自动添加 `wasm32-unknown-unknown` 作为目标代码，然后构建应用，应用将默认在 [http://[::1\]:8000](http://[::1]:8000/) 被访问。

* 结果如下

  ![image-20200328233848478](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200328233848478.png)

## 总结

* 可见通过在Rust中引用yew库实现web前端应用是可行的

## 参考文献

https://yew.rs/docs/v/zh_cn/  Yew Docs