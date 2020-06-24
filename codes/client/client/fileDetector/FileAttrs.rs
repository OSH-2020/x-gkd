pub struct FileAttrs {
    name: String,
    path: String,
    attr: String,
    noa: i32,
}
impl FileAttrs {
    pub fn init(name: String, path: String, attr: String, noa: i32) -> Self {
        FileAttrs {
            name: name,
            path: path,
            attr: attr,
            noa: noa,
        }
    }
}