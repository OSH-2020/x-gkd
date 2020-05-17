## 纠删码相关说明文档

### 一、相关文档链接

Github项目：https://github.com/darrenldl/reed-solomon-erasure

Rust docs：[Crate reed_solomon_erasure](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/)

### 二、使用

#### Rust usage（摘自Github）

Add the following to your `Cargo.toml` for the normal version (pure Rust version)

```
[dependencies]
reed-solomon-erasure = "4.0"
```

or the following for the version which tries to utilise SIMD

```
[dependencies]
reed-solomon-erasure = { version = "4.0", features = "simd-accel" }
```

and the following to your crate root

```
extern crate reed_solomon_erasure;
```
示例程序：

```rust
#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;
// or use the following for Galois 2^16 backend
// use reed_solomon_erasure::galois_16::ReedSolomon;

fn main () {
    let r = ReedSolomon::new(3, 2).unwrap(); // 3 data shards, 2 parity shards

    let mut master_copy = shards!(
        [0, 1,  2,  3],
        [4, 5,  6,  7],
        [8, 9, 10, 11],
        [0, 0,  0,  0], // last 2 rows are parity hards
        [0, 0,  0,  0]
    );

    // Construct the parity shards
    r.encode(&mut master_copy).unwrap();

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = master_copy.iter().cloned().map(Some).collect();

    // We can remove up to 2 shards, which may be data or parity shards
    shards[0] = None;
    shards[4] = None;

    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    assert!(r.verify(&result).unwrap());
    assert_eq!(master_copy, result);
}
```

#### 可能出现的问题

windows 环境中，下载`reed-solomon-erasure` 可能出现的问题：

![](C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 133749.png)

解决方法：

将以下内容添加到 `Cargo.toml` 中

```
[http]
check-config = false
```

### 三、struct reed solomon erasure::ReedSolomon的说明

链接：Rust docs ：Struct [reed_solomon_erasure](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/index.html)::[ReedSolomon](https://docs.rs/reed-solomon-erasure/4.0.2/reed_solomon_erasure/struct.ReedSolomon.html)

```
pub struct ReedSolomon<F: Field> { /* fields omitted */ }
```

该结构体是一个 Reed-Solomon erasure code encoder/decoder.

注：

- data shards 数据碎片 parity shards 奇偶校验碎片

- 方法结尾处有 `_sep` 表示对数据碎片采用不变的引用，对奇偶校验碎片采用可变的引用。（对数据碎片只读）

- 方法结尾处有`_single` 表示 shard by shard encoding，即奇偶校验碎片是一次使用一个数据碎片部分构建的，**容易错误使用**。

| methods            | 说明 |
| ------------------ | ---- |
|**new**				| pub fn new(data_shards: usize,  parity_shards: usize) -> Result<ReedSolomon\<F>,Error> |
| data_shard_count   | 返回data shard的数目 |
| parity_shard_count | 返回parity shard的数目 |
| total_shard_count  | 返回total shard的数目 |
| **encode**         | Constructs the parity shards. （The slots where the parity shards sit at will be overwritten.） |
| encode_sep         | Constructs the parity shards using a read-only view into the data shards. （The slots where the parity shards sit at will be overwritten.） |
| encode_single      | 比encode方法多一个参数i_data索引，只用该索引指向的数据碎片构建奇偶校验碎片 |
| encode_single_sep  |      |
| **reconstruct**    | Reconstructs all shards |
| reconstruct_data   | Reconstructs only the data shards. |
| **verify**         | 核验奇偶校验碎片是否正确 |
| verify_with_buffer | 每次verify时，需开辟一段内存空间，存储根据数据碎片计算出的奇偶校验碎片内容，再与原内容比较，以判断是否正确。verify_with_buffer 是将buffer指定为这段内存空间，避免重复开辟。 |

注：加粗的为常用方法

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151419.png" style="zoom: 50%;" />

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151349.png" style="zoom: 50%;" />



<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151435.png" style="zoom:50%;" />

![](C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 151455.png)

### 四、需要改写的部分

上述方法不是针对文件的，原17级项目也改写了backblaze的 encoder和decoder模块，我们也需要改写成适合文件的。

原 17 级设计报告中涉及纠删码模块的部分如下：

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 141609.png" style="zoom: 60%;" />

<img src="C:\Users\12935\Pictures\Screenshots\批注 2020-05-17 141624.png" style="zoom:60%;" />

参考 17 级源代码中的encoder，他们的做法是：FILE -> FileInputStream -> InputStream -> shards \[][] -> 调用 -> FileOutputStream -> OutputStream -> write 到指定文件中

改写难点：

- Rust 文件输入流处理

- backblaze 的纠删码算法中的方法定义可能与 Rust中的纠删码算法不太一样。