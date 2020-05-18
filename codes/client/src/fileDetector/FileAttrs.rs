crate fileDetector

/**
 * 文件属性
 */
struct FileAttrs {
    name: String,
    path: String,
    attr: String,
    noa: i32,
}
fn FileAttrs(name: String, path: String,
    attr: String, noa: i32) -> Self {
    FileAttrs {
        name: name,
        path: path,
        attr: attr,
        noa: noa,
    }
}