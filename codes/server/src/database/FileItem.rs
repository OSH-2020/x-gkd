pub struct FileItem {
    id: i32,
    name: String,
    path: String,
    attribute: String,
    time: String,
    noa: i32,
    isFolder: bool,
}

impl FileItem {
    fn init(id: i32, name: String, path: String, attribute: String,
        time: String, noa: i32, isFolder: bool) -> self {
            FileItem {
                id,
                name,
                path,
                attribute,
                time,
                noa,
                isFolder,
            }
        }

    pub fn init_2(name: String, path: String, attribute: String,
        time: String, noa: i32, isFolder: bool) -> self {
            FileItem {
                id: 0,
                name,
                path,
                attribute,
                time,
                noa,
                isFolder,
            }
        }

    pub fn getId(&mut self) -> i32 {
        self.id
    }

    pub fn getName(&mut self) -> String {
        self.name
    }

    pub fn setName(&mut self, name: String) {
        self.name = name;
    }

    pub fn getPath(&mut self) -> String {
        self.path
    }

    pub fn setPath(&mut self, path:String) {
        self.path = path;
    }

    pub fn getAttribute(&mut self) -> String {
        self.attribute = attribute;
    }

    pub fn setAtrribute(&mut self, attribute: String) {
        self.attribute = attribute;
    }

    pub fn getTime(&mut self) {
        self.time
    }

    pub fn setTime(&mut self, time:String) {
        self.time = time;
    }

    pub fn getNoa(&mut self) -> i32{
        self.noa
    }

    pub fn setNoa(&mut self, noa:i32) {
        self.noa = noa;
    }

    pub fn isFolder(&mut self) -> bool {
        self.isFolder
    }

    pub fn setFolder(&mut self, isFolder:bool) {
        self.isFolder = isFolder;
    }
}