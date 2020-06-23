/*mod FileAttrs;
mod FileUtil;
mod FolderScannner;*/

use std::path::PathBuf;
use std::net::TcpStream;
use std::string::String;
use std::io::prelude::*;
use std::fs::read_to_string;

struct FileUploader {
    serverIP: String,
    server_port: u16,
    tmpFragmentFolder: PathBuf,
    to_server: Option<TcpStream>,
    socket: TcpStream,
    //outToServer: String,
    //inFromServer: String,
    connecting: bool,
}

impl FileUploader {
    /*pub fn init(f:PathBuf, ip: String, port:u16) -> Self{
        FileUploader {
            serverIP: ip.clone(),
            server_port: port,
            tmpFragmentFolder: f,
            connecting = false,
            to_server: Err,
            socket: ,
            //outToServer: String::new(),
            //inFromServer: String::new(),
        }
    }*/

    pub fn checkFolders(mut self, addr: Vec<String>) -> bool{
        if !self.connecting
            {return false;}
        self.socket.write_fmt(format_args!("6 0 {}\n", addr.len()));
        self.socket.flush();

        let mut i = 0;
        while i < addr.len() {
            let c = addr[i].chars();
            let mut j = -1;
            let mut n = 0;
            for cur in c {
                if cur == '/' {j = n;}
                n = n+1;
            }
            if j==-1
                {self.socket.write_fmt(format_args!("/ {}\n", &addr[i]));}
            else {
                let mut number = 0;
                let ch = addr[i].chars();
                for cur in ch {
                    self.socket.write_fmt(format_args!("{}\n", cur));
                    if number == j {self.socket.write_fmt(format_args!("/ "));}
                    number = number + 1;
                }
                self.socket.write_fmt(format_args!("\n"));
            }
            self.socket.flush();
            i = i + 1;
        }
        let re = ['r','e','c','e','i','v','e','d','!'];
        let mut i = 0;
        let mut input_buf = String::new();
        self.socket.read_to_string(&mut input_buf);
        for c in input_buf.chars() {
            if c == re[i] {i = i+1;}
            else {return false;}
        }
        return true;
    }

    pub fn registerFile(&mut self, fa: PathBuf) -> i32 {
        if !self.connecting {return -2;}
        self.socket.write_fmt(format_args!("4 0 "));
        //self.socket.write_fmt(format_args!("{} false\n", fa));//other output
        self.socket.flush();

        let mut sentence = String::new();
        self.socket.read_to_string(&mut sentence);
        let mut input: Vec<char> = Vec::new();
        for cha in sentence.chars() {
            if cha == ' ' {break;}
            else {input.push(cha);}
        }
        let mut num = 0;
        for cha in "FileId:".chars() {
            if cha != input[num] {return -2;}
            num = num+1;
        }
        let mut inputline = String::new();
        num = 0;
        for cha in sentence.chars() {
            if cha == ' ' {num = num + 1;}
            if num == 2 {break;}
            else if num == 1 {inputline.push(cha);}
        }
        let integer = inputline.parse::<i32>().unwrap();
        return integer;
    }

    pub fn pushFragment(&mut self, fileId: i32, fragmentNum: i32, fragmentCount: i32) -> bool {
        let mut status = false;
        let mut sentence = String::new();
        if !self.connecting {return false;}
        
        self.socket.write_fmt(format_args!("5 {} {} {}\n", fileId, fragmentNum, fragmentCount));
        self.socket.flush();

        let re = ['r','e','c','e','i','v','e','d','!'];
        let mut i = 0;
        let mut inFromServer = String::new();
        self.socket.read_to_string(&mut inFromServer);
        for c in inFromServer.chars() {
            if c == re[i] {i = i+1;}
            else {return false;}
        }

        //status = connect.FileTransporter.sendFile(f, inFromServer, outToServer);
        //没有理解好，还未转换

        if status {
            let re = ['r','e','c','e','i','v','e','d','!'];
            let mut i = 0;
            for c in inFromServer.chars() {
                if c == re[i] {i = i+1;}
                else {return false;}
            }
        }
        return true;
    }

    pub fn createConnection(&mut self) {
        if let Ok(connect_socket) = TcpStream::connect((&self.serverIP[..], self.server_port)) {
            self.to_server = Some(connect_socket);
            println!("Connect to server successfully(control)!");
            self.connecting = true;
        } else {
            println!("Couldn't connect to server...");
            self.connecting = false;
        }

        match &self.to_server{
            None => println!("Error! server not connected..."),
            Some (socket) => {
                //self.socket = socket.clone();
            }
        }
    }
}