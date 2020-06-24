### wasm-game-of-life

在 `pqz可行性分析.md` 中有关于本部分更详细的内容。

* 需要工具

  * Rust toolchain (rustup, rustc, cargo)
  * wasm-pack
  * cargo-generate
  * npm

* 主要流程

  * 下载模板

    `cargo generate --git https://github.com/rustwasm/wasm-pack-template`

    `project-name`

  * build

    `wasm-pack build` （可以认为相当于一次编译。）

  * putting into a web page

    `npm init wasm-app www`

  * 将 npm 的默认 hello-wasm-pack 包修改为本地项目

    在 `project-name/www/package.json` 文件，"devDependencies" 代码块前加入：

    ```
      "dependencies": {
        "project-name": "file:../pkg"
      },
    ```

    修改 `project-name/www/index.js` 文件，将 "hello-wasm-pack" 改为本地的 "project-name" ：

    ```
    import * as wasm from "project-name";
    ```

  * 下载依赖

    在 www 目录下运行 `npm install`

  * 在 http://localhost:8080/ 本地运行

    `npm run start`

* project-name/src/lib.rs 是编译时的根文件，其中定义结构体和对应方法或函数，由 project-name/www/index.js 调用，最终由 project-name/www/index.html 中 `<script src="./bootstrap.js"></script>` 一行调用执行 index.js 。

* lib.rs 中结构体或方法定义之前加上 \#[wasm_bindgen] 注解，才能将对应代码块中的内容暴露给 JavaScript 。在 project-name/www/index.js 中可以使用 `import {...} from "project-name";` 导入。



### Rust Crates 能否与 WebAssembly 共同工作 

可能导致不能与 WebAssembly 共同工作的特性：

* C 库及系统库的依赖

  含有 `#![no_std]` 注解的代码可能适宜与 WebAssembly 共同工作。

* 文件 IO

  > WebAssembly does not have access to a file system, so crates that assume the existence of a file system — and don't have wasm-specific workarounds — will not work.

* 同步 IO

  > There is only asynchronous I/O on the Web.

  应当尽可能在暴露给 wasm 的内容中避免直接的 IO 操作，如果必须进行 IO 操作，不能是同步操作。

* 多线程

  Multithreading Rust and wasm：

  > Instead of providing a full library experience the threads proposal is instead specifying the fundamental building blocks upon which you can build a threading library.



可能适宜与 WebAssembly 共同工作的特性：

* 提供特定算法和数据结构的 Crate
* `#![no_std]` （ wasm-game-of-life 中有使用 std::fmt，但对应代码块没有加上 \#[wasm_bindgen] 注解）
* Parsers （语法分析器）
* 处理文本的 Crate
* Rust Patterns （这个具体是什么我还没有搞清楚）



其他

* * `cargo build --target wasm32-unknown-unknown` 

    这条指令能够运行成功，则该 Crate 可能支持 WebAssembly 。

  * 在 CI script 中运行

    `rustup target add wasm32-unknown-unknown`
    `cargo check --target wasm32-unknown-unknown`

    * Travis CI ：持续集成服务，是构建和测试的自动化工具

  * 在 Node.js 中测试

* 编译为 WebAssembly 时可能需要添加的依赖：

  > If you need to interact with the outside world (i.e. you can't have library consumers drive that interaction for you) 

  ```
  [target.'cfg(target_arch = "wasm32")'.dependencies]
  wasm-bindgen = "0.2"
  js-sys = "0.3"
  web-sys = "0.3"
  ```




### Compling Rust to WebAssembly

* #[wasm_bindgen] extern 代码块引入外部的 java 函数，供 Rust 代码调用

  ```
  #[wasm_bindgen]
  extern {
  	pub fn alert(s: &str);
  }
  ```

* #[wasm_bindgen] pub 将 Rust 函数暴露给 java

  ```
  #[wasm_bindgen]
  pub fn greet(name: &str) {
  	alert(&format!("Hello, {}!", name));
  }
  ```

  



### 一些想法

* 网页前端部分需要与 WebAssembly 交互的地方可能比较少，或许在 html 文件中加入 `<script src="./bootstrap.js"></script>` 一行，或在其他更高层的代码文件中调用 WebAssembly 内容就足够。
* 可能考虑只纠删码编解码计算部分使用 WebAssembly ……但传入传出的参数需要再详细考虑，因为 WebAssembly 数据复制和 serializing and deserializing 开销可能较大。
* 将 WebAssembly 部分作为一个 lib crate ，在其他文件中调用这个库完成计算。



### 参考

[Rust-wasm book](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html)

[Multithreading Rust and wasm](https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html)

[Travis CI 教程](http://www.ruanyifeng.com/blog/2017/12/travis_ci_tutorial.html)

[Compling Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)