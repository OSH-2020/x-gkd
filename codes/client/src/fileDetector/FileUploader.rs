/*mod FileAttrs;
mod FileUtil;
mod FolderScannner;*/

use std::fs::File;
use std::path::Path;
use std::net::TcpStream;
use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;

struct FileUploader {
    serverIP: String,
    ServerPort: i32,
    tmpFragmentFolder: File,
    tmpFragmentFolderPath: Path,
    toServer: Option<TcpStream>,
    outToServer: String,
    inFromServer: String,
}

impl FileUploader {
    pub fn init(f:File, ip: String, port:i32) -> Self{
        FileUploader {
            serverIP: ip,
            serverPort: port,
            tmpFragmentFolder: f,
            outToServer: String::new(),
            inFromServer: String::new(),
        }
    }

    pub fn checkFolders(&mut self, &addr: Vec<String>) -> bool{
        if (!self.createConnection())
            {return false;}
        write!(self.toServer,"6 0 {}\n", addr.length);
        self.toServer.flush();

        let mut i = 0;
        while i < addr.len() {
            let c = &addr[i].chars();
            let mut j = -1;
            let mut n = 0;
            for cur in c {
                if(cur == '/') {j = n;}
                n = n+1;
            }
            if (j==-1)
                {write!(self.toServer,"/ {}\n", &addr[i]);}
            else {
                let mut number = 0;
                for cur in c {
                    write!(self.toServer,"{}\n", cur);
                    if(number == j) {write!(self.toServer,"/ ");}
                    number = number + 1;
                }
                write!(self.toServer,"\n");
            }
            self.toServer.flush();
        }
        let re = {r,e,c,e,i,v,e,d,!};
        let i = 0;
        for c in self.inFromServer.chars() {
            if(c == re.get(i)) {i = i+1;}
            else {return false;}
        }
        return true;
    }

    pub fn registerFile(&mut self, fa: Path) -> i32 {
        if(!self.createConnection()) {return -2;}
        write!(self.toServer,"4 0 ");
        write!(self.toServer,"{} false\n", fa);//are the others neccessary?
        self.toServer.flush();

        let sentence = read_to_string(&self.inFromServer);
        let mut input: Vec<char> = Vec::new();
        for cha in sentence {
            if(cha == ' ') {break;}
            else {input.push(cha);}
        }
        let mut num = 0;
        for cha in "FileId:" {
            if(cha != &input[num]) {return -2;}
            num = num+1;
        }
        let mut inputline = String::new();
        num = 0;
        for cha in sentence {
            if(cha == ' ') {num = num + 1;}
            if(num == 2) {break;}
            else if(num == 1) {inputline.push_str(cha);}
        }
        let integer = inputline.parse::<i32>().unwrap();
    }

    pub fn pushFragment(&mut self, fileId: i32, fragmentNum: i32, fragmentCount: i32) -> bool {
        let mut status = false;
        let mut sentence = String::new();
        if(!self.createConnection()) {return false;}
        
        write!(self.toserver, "5 {} {} {}\n", fileId, fragmentNum, fragmentCount);
        self.toserver.flush();

        let re = {r,e,c,e,i,v,e,d,!};
        let i = 0;
        for c in self.inFromServer.chars() {
            if(c == re[i]) {i = i+1;}
            else {return false;}
        }

        //status = connect.FileTransporter.sendFile(f, inFromServer, outToServer);
        //没有理解好，还未转换

        if(status) {
            let re = {r,e,c,e,i,v,e,d,!};
            let i = 0;
            for c in self.inFromServer.chars() {
                if(c == re.get(i)) {i = i+1;}
                else {return false;}
            }
        }
        return true;
    }

    pub fn createConnection(&mut self) -> bool {
        if(self.serverIP == 0)
            {return false;}
        if let Ok(connect_socket) = TcpStream::connect((&self.server_ip[..], self.control_port)) {
            self.toserver = Some(connect_socket);
            println!("Connect to server successfully(control)!");
        } else {
            println!("Couldn't connect to server...");
        }
        self.toServer.set_nonblocking(true).unwrap();

        let mut input_buf = String::new();
        match &mut self.toserver{
            None => println!("Error! server not connected..."),
            Some (socket) => {
                let socket_read = socket.try_clone().expect("clone failed...");
                let mut in_from_server = BufReader::new(socket_read);
                in_from_server.read_line(&mut input_buf).unwrap();
                let input_buf = input_buf.trim();
                self.inFromServer = input_buf;
            }
        }
    }

    pub fn closeConnection(&mut self) {
        self.toServer.close();
        self.inFromServer.close();
    }

}
