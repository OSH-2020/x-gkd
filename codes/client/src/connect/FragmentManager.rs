//use std::net::{TcpStream, Ipv4Addr, Shutdown};
use std::net::TcpStream;
use std::string::String;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;
//use std::{thread, time};
use std::fs::{File, remove_file};
//use std::ptr::null;
use std::path::Path;
//use std::option::NoneError;
use std::option::Option;


struct FragmentManager{
    fragmentFolder : File,
    serverIP : String,
    serverPort : i32,
    controlPort : u16,
    toServer : Option<TcpStream>,
    inFromServer : BufReader<TcpStream>,
    //DateOut/inputStream ??
    requestID : i32,
    fragmentID : i32,
    Type : i32,
}


impl FragmentManager {
    pub fn new(rId : i32, fId : i32, t : i32, f : File, l : TcpStream)->FragmentManager{
        FragmentManager{
            fragmentFolder : f,
            serverIP : String :: new(),
            serverPort : -1,
            controlPort : 0,
            toServer : None,
            inFromServer: BufReader :: new(l),
            requestID : rId,
            fragmentID : fId,
            Type : t//type为Rust关键字，改为大写开头
        }
    }

    pub fn init0(&mut self, tmp : File){
        self.fragmentFolder = tmp;
    }

    pub fn run(){
        // 暂不进行并发数据操作
        // submit();
    }

    pub fn submit(&mut self) -> bool {
        let mut status = true;
        if self.serverIP.len() == 0 {
            return false;
        }
        if let Ok(connect_socket) = TcpStream::connect((&self.serverIP[..], self.controlPort)) {
            self.toServer = Some(connect_socket);//忽略了setKeepAlieve和setsoTimeout，未找到rust中对应的长连接和超时连接的处理函数
            println!("Connect to server successfully(data)!");
            if self.Type == 1 {
                status = self.sendFragment();
            } else if self.Type == 2 {
                status = self.recvFragment();
            } else if self.Type == 3 {
                status = self.deleteFragment();
            }
        } else {
            println!("Cannot connect to server");
            status = false;
        }
        return status;
    }

    pub fn init(&mut self, f : File, ip : String, port : i32) {
        self.fragmentFolder = f;
        self.serverIP = ip;
        self.serverPort = port;
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                let socket_read = socket.try_clone().expect("clone failed");
                let mut inFromServer = BufReader :: new(socket_read);
            }
        }
    }
    //以下函数未实现throw exceptions
    fn sendFragment(&mut self) -> bool {
        let mut status = true;
        let mut sentense = String :: new();
        //let s = self.fragmentFolder + "/" + self.fragmentID;
        //可能会根据运行平台的不同添加/,分为posix和windows
        //一个问题是无法获取文件的绝对路径，没有相应的Path方法
        let s = self.fragmentID.to_string();
        let path = Path::new(&s);
        let mut f = File::create(path);
        if !f.is_ok() {//存疑
            panic!("Error happens on File");
        }

        //@SuppressWarnings("deprecation")
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                self.inFromServer.read_line(&mut sentense).unwrap();
                let recv = String :: from("received!");
                if !sentense.eq(&recv) {
                    return false;
                }
                let mut status = true;
                //let mut status = FileTransporter.sendFile 需要另一个函数FileTransporter
                if status {
                    self.inFromServer.read_line(&mut sentense).unwrap();
                    if !sentense.eq(&recv) {
                        return false;
                    }
                }
            }
        }
        return status;

    }

    fn recvFragment(&mut self) -> bool {
        let s = self.fragmentID.to_string();
        let path = Path::new(&s);
        let mut f = File::create(path);
        if f.is_ok() {
            remove_file(path);//路径的问题需要调试
        }
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                //if(FIleTransporter.recvFIle) 调用另一个函数
            }
        }
        return true;//为了通过编译加的一句，后续删除

    }

    fn deleteFragment(&mut self) -> bool {
        let s = self.fragmentID.to_string();
        let path = Path::new(&s);
        let mut f = File::create(path);
        if f.is_ok() {
            remove_file(path);
        }

        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                //SuppressWarngings
                let mut sentense = String ::new();
                self.inFromServer.read_line(&mut sentense).unwrap();
                let recv = String :: from("received!");
                if sentense.eq(&recv) {
                    return true;
                }else {
                    return true;
                }
            }

        }
        return true;//不知道为什么最后不加返回值就报错，后续需要处理


    }


    fn errorHandler(Type : i32){
        return;
    }



}


