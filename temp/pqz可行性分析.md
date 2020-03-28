# Rust+WebAssembly+JS+Node.js可行性分析

* 通过研究和尝试将Rust实例转化为WebAssembly，调用相应的库并将其放在Node.js（基于Chrome V8引擎的JavaScript运行环境）上运行，来说明此套流程的可行性
* JavaScript的垃圾收集堆与WebAssembly的Rust值所在的线性内存空间不同。WebAssembly当前无法直接访问垃圾收集的堆
* JavaScript可以读取和写入到WebAssembly线性内存空间
* 一般而言，良好的JavaScript↔WebAssembly接口设计通常是将大型，长期存在的数据结构实现为Rust类型，并驻留在WebAssembly线性内存中，并作为不透明的句柄暴露给JavaScript。JavaScript调用导出的WebAssembly函数，这些函数采用这些不透明的句柄，转换其数据，执行大量计算，查询数据并最终返回小的可复制结果。通过仅返回较小的计算结果，避免了在JavaScript垃圾回收堆和WebAssembly线性内存之间来回复制或串行化所有内容。

## 工具链

* **Rust工具链**

  * 标准的Rust工具链：rustup,rustc,cargo

* **wasm-pack**

  * 用于构建、测试和发布由Rust生成的WebAssembly，并与Java、Web和Node.js进行交互操作

* **wasm-opt**

  * 读取WebAssembly作为输入，对其进行转换，优化和检测，并输出转换过后的WebAssembly

* **wasm2js**

  * 将WebAssembly便以为"almost asm.js"，用于支持没有WebAssembly实现的浏览器

* **wasm-gc**

  * 对WebAssembly模块进行垃圾回收，删除所有不需要的导出，导入，函数等

* **wasm-snap**

  * 用unreachable指令替换WebAssembly函数的主体

* **twiggy**、**wasm-objdump**、**wasm-nm**

  * 用于检查.wasm二进制文件

* **cargo-generate**

  * 利用预先存在的git存储库作为模板，可以快速运行新的Rust项目

  * > cargo install cargo-generate

* **npm**

  * JavaScript的软件包管理器，用于安装和运行JavaScript捆绑器和开发服务器

* **Crates**

  * 与JavaScript和DOM进行交互
    * **wasm-bindgen**：促进Rust和JavaScript之间的高级交互。它允许将JavaScript内容导入Rust，并将Rust内容导出到JavaScript；定义了如何跨边界使用复合结构的共识。它涉及将Rust结构装箱，将指针包装在JavaScript类中以提高可用性，或将其索引到Rust中的JavaScript对象表中。
    * **wasm-bindgen-futures**:连接JavaSript Promise和Rust Future的桥梁。它可以双向转换，在Rust中使用异步任务时非常有用，并且可以与DOM事件和I/O操作进行交互
    * **js-sys**:用于所有的JavaScript全局类型和方法的Raw wasm-bindgen，如Object，Function，eval等
    * **web-sys**:wasm-bindgen中所有Web API的原始导入，如DOM操作setTimeout，Web GL ，Web Audio
  * 错误报告和记录
    * **console_error_panic_hook**、**console_log**
  * 动态分配
    * **wee_alloc**
  * 解析和生成.wasm二进制文件
    * **parity-wasm**：用于序列化，反序列化和构建.wasm二进制文件的低级WebAssembly格式库
    * **wasmparser**：一个简单的事件驱动型库，用于解析WebAssembly二进制文件
  * 解释和编译WebAssembly
    * **wasmi**：来自Parity的可嵌入WebAssembly解释器
    * **cranelift-wasm**:将WebAssembly编译为本机主机的机器代码

* 模板

  * **wasm-pack-template**：用于搭配wasm-pack启动Rust和WebAssembly项目
  * **create-wasm-cpp**：用于JavaScript项目，从Rust搭配wasm-pack创建的npm中获取包
  * **rust-webpack-template**：预先配置了所有样板，用于将Rust编译为WebAssembly并将其直接挂钩到Webpack的Webpack构建管道中的rust-loader

## 过程

* 从github上克隆项目模板

  > cargo generate --git https://github.com/rustwasm/wasm-pack-template

  * 关键文件解析
    * **Cargo.toml**：指定了cargo Rust的包管理器和构建工具的依赖项和元数据
    * **/scr/lib.rs**:正在编译为WebAssembly的Rust crate的根文件，用于wasm-bindgen与JavaScript进行交互
    * **/scr/utils.rs**:提供了通用程序,使Rust编译为WebAssembly的过程更加轻松

* 在项目目录中运行

  > wasm-pack build

* 构建完成后，可在pkg目录中找到相应的文件

  ```
  pkg/
  ├── package.json
  ├── README.md
  ├── wasm_game_of_life_bg.wasm
  ├── wasm_game_of_life.d.ts
  ├── wasm_game_of_life.js
  └──...
  ```
  * **wasm_game_of_life_bg.wasm**：WebAssembly的二进制文件，由Rust编译器从Rust的源代码生成，包含所有的Rust函数和数据的wasm版本
  * **wasm_game_of_life.js**:由wasm-gindgen JavaScript胶水生成并包含JavaScript胶水，用于将DOM和JavaScript函数导入Rust，并将WebAssembly函数API公开给JavaScript
  * **wasm_game_of_life.d.ts**：包含JavaScript胶水的TypeScript类型声明
  * **package.json**：包含和生成的JavaScript和WebAssembly包有关的元数据

* 放入网页

  * 在wasm-game-of-life中运行命令

    > npm init wasm-app example

  * 打开example子目录可以看到如下文件

    ```
    wasm-game-of-life/example/
    ├── bootstrap.js
    ├── index.html
    ├── index.js
    ├── LICENSE-APACHE
    ├── LICENSE-MIT
    ├── package.json
    ├── README.md
    ├── webpack.config.js
    └── ...
    ```

  * 分析

    * **package.json**:自带预先配置有webpack和webpack-dev-server 依赖,以及初始的hello-wasm-pack

    * **webpack.config.js**：配置webpack及其本地开发服务器

    * **index.html**：网页的根HTML文件，负载了bootstrap.js

      ```html
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="utf-8">
          <title>Hello wasm-pack!</title>
        </head>
        <body>
          <script src="./bootstrap.js"></script>
        </body>
      </html>
      ```

    * **index.js**：Web页面JavaScript的主要入口点，用于导入hello-wasm-pack的npm包，其中包含默认 wasm-pack-template的已编译WebAssembly和JavaScript胶水，然后调用hello-wasm-pack的greet函数

      ```java
      import * as wasm from "hello-wasm-pack";
      
      wasm.greet();
      ```

* 安装依赖项

  * 在example子目录中运行如下指令，用于安装webpack JavaScript捆绑器及其开发服务器

    > npm install

* 使用本地的example包

  * 在/example/package.json中的dependencies中加入

    > "wasm-game-of-life": "file:../pkg"

  * 修改/example/index.js

    ```java
    import * as wasm from "wasm-game-of-life";
    
    wasm.greet();
    ```

  * 重新执行

    > npm install

* 开启一个新的终端，并在example目录中运行

  > npm run start

  并将浏览器导航到http://localhost:8080/，即可看到写有“Hello,wasm-game-of-life”的提示框

* Canvas API的使用，直接从内存渲染到Canvas（不再使用Unicode文本）

  * 在index.html内部进行如下替换

    ```html
    <body>
      <canvas id="game-of-life-canvas"></canvas>
      <script src='./bootstrap.js'></script>
    </body>
    ```

  * 重新在wasm-game-of-life中依次执行如下命令，并将浏览器导航到http://localhost:8080/

    > wasm-pack build
    >
    > npm run start

    结果如下

    ![image-20200327202000187](C:\Users\dell\AppData\Roaming\Typora\typora-user-images\image-20200327202000187.png)

  * Rust代码如下

    ```Rust
    mod utils;
    
    use wasm_bindgen::prelude::*;
    
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    
    #[wasm_bindgen]
    extern {
        fn alert(s: &str);
    }
    
    #[wasm_bindgen]
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cell {
        Dead = 0,
        Alive = 1,
    }
    
    #[wasm_bindgen]
    pub struct Universe {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
    }
    
    impl Universe {
        fn get_index(&self, row: u32, column: u32) -> usize {
            (row * self.width + column) as usize
        }
    
        // ...
    }
    
    impl Universe {
        // ...
    
        fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
            let mut count = 0;
            for delta_row in [self.height - 1, 0, 1].iter().cloned() {
                for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }
    
                    let neighbor_row = (row + delta_row) % self.height;
                    let neighbor_col = (column + delta_col) % self.width;
                    let idx = self.get_index(neighbor_row, neighbor_col);
                    count += self.cells[idx] as u8;
                }
            }
            count
        }
    }
    
    /// Public methods, exported to JavaScript.
    #[wasm_bindgen]
    impl Universe {
        pub fn tick(&mut self) {
            let mut next = self.cells.clone();
    
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);
    
                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Cell::Alive, x) if x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Cell::Alive, x) if x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Cell::Dead, 3) => Cell::Alive,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };
    
                    next[idx] = next_cell;
                }
            }
    
            self.cells = next;
        }
    
        // ...
    }
    
    use std::fmt;
    
    impl fmt::Display for Universe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for line in self.cells.as_slice().chunks(self.width as usize) {
                for &cell in line {
                    let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                    write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }
    
            Ok(())
        }
    }
    
    /// Public methods, exported to JavaScript.
    #[wasm_bindgen]
    impl Universe {
        // ...
    
        pub fn new() -> Universe {
            let width = 64;
            let height = 64;
    
            let cells = (0..width * height)
                .map(|i| {
                    if i % 2 == 0 || i % 7 == 0 {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                })
                .collect();
    
            Universe {
                width,
                height,
                cells,
            }
        }
    
        pub fn render(&self) -> String {
            self.to_string()
        }
    }
    
    /// Public methods, exported to JavaScript.
    #[wasm_bindgen]
    impl Universe {
        // ...
    
        pub fn width(&self) -> u32 {
            self.width
        }
    
        pub fn height(&self) -> u32 {
            self.height
        }
    
        pub fn cells(&self) -> *const Cell {
            self.cells.as_ptr()
        }
    }
    ```

  * 可见，通过调用wasm-bindgen等库来实现JavaScript和WebAssembly之间的通信是可行的，这也为后续开发奠定了基础。