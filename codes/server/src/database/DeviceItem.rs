pub struct DeviceItem {
    id: i32,
    ip: String,
    port: i32,
    isOnline: bool,
    rs: i32,
}

impl DeviceItem {
    pub fn init(id: i32, ip: String, port: i32, isOnline: bool, rs: i32) -> Self{
        DeviceItem {
            id: id,
            ip: ip,
            port: port,
            isOnline: isOnline,
            rs: rs,
        }
    }

    pub fn getId(&mut self) -> i32 {
        self.id
    }

    pub fn getIp(&mut self) -> String {
        self.ip
    }

    pub fn setIp(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn getPort(&mut self) -> i32 {
        self.port
    }

    pub fn setPort(&mut self, port:i32) {
        self.port = port;
    }

    pub fn isOnline(&mut self) -> bool {
        self.isOnline
    }

    pub fn setIsOnline(&mut self, isOnline:bool) {
        self.isOnline = isOnline;
    }

    pub fn getRs(&mut self) -> i32 {
        self.rs
    }

    pub fn setRs(&mut self, rs: i32) {
        self.rs = rs;
    }
}