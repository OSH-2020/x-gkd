

pub struct RequestItem {
    id: i32,
    type: i32,
    fragmentId: i32,
    deviceId: i32,
}

impl RequestItem {
    fn init(id: i32, type: i32, fid: i32, did: i32) -> self {
        RequestItem {
            id,
            type,
            self.fragmentId: fid,
            self.deviceId: did,
        }
    }

    pub fn init_2(type: i32, fid: i32, did: i32) -> self {
        RequestItem {
            id: 0,
            type,
            fragmentId: fid,
            deviceId: did,
        }
    }

    pub fn getId(&mut self) -> i32 {
        self.id
    }

    pub fn getType(&mut self) -> i32 {
        self.type
    }

    pub fn setType(&mut self, type: i32) {
        self.type = type;
    }

    pub fn getFragmentId(&mut self) -> i32 {
        self.fragmentId
    }

    pub fn setFragmentId(&mut self, fragmentId: i32) {
        self.fragmentId = fragmentId;
    }

    pub fn getDeviceId(&mut self) -> i32 {
        self.deviceId
    }

    pub fn setDeviceId(&mut self, id: i32) {
        self.deviceId = id;
    }
}