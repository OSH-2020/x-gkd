mod FileAttrs;
mod FileUtil;
mod FolderScannner;

use std::fs::File;
use std::path::Path
use mio::net::TcpStream;
use std::io::TcpStream;

pub struct FileUploader {
    serverIP: String,
    ServerPort: i32,
    tmpFragmentFolder: File,
    tmpFragmentFolderPath: Path,
    toServer: TcpStream,
    outToServer: String,
    inFromServer: String,
}

impl FileUploader {
    pub fn init(f:File, ip: String, port:i32) -> Self{
        FileUploader {
            serverIP: ip,
            serverPort: port,
            tmpFragmentFolder: f,
            outToServer: null,
            inFromServer: null,
        }
    }

    pub fn checkFolders(&addr: Vec<String>) -> bool{
        if (!createConnection())
            return false;
        write!(toServer,"6 0 {}\n", addr.length);
        toServer.flush();

        let mut i = 0;
        while i < addr.len() {
            let c = &addr[i].chars();
            let mut j = -1;
            let mut n = 0;
            for cur in c {
                if(cur == '/') j = n;
                n++;
            }
            if (j==-1)
                write!(toServer,"/ {}\n", &addr[i]);
            else {
                let mut number = 0;
                for cur in c {
                    write!(toServer,"{}\n", cur);
                    if(number == j) write!(toServer,"/ ");
                    number++;
                }
                write!(toServer,"\n");
            }
            toServer.flush();
        }
        let re[] = "received!";
        let i = 0;
        for c in inFromServer.chars() {
            if(c == re[i]) i++;
            else return false;
        }
        return true;
    }

    fn registerFile(fa: path) -> i32 {
        if(!createConnection()) return -2;
        write!(toServer,"4 0 ");
        write!(toServer,"{} false\n", fa);//are the others neccessary?
        toServer.flush();

        let sentence = read_to_string(&inFromServer);
        let mut input: Vec<char> = Vec::new();
        for cha in sentence {
            if(cha == ' ') break;
            else input.push(cha);
        }
        let mut num = 0;
        for cha in "FileId:" {
            if(cha != &input[num]) return -2;
            num++;
        }
        let mut inputline = String::new();
        num = 0;
        for cha in sentence {
            if(cha == ' ') num++;
            if(num == 2) break;
            else if(num == 1) inputline.push_str(cha);
        }
        let integer = inputline.parse::<i32>().unwrap();
    }

    fn pushFragment(fileId: i32, fragmentNum: i32, fragmentCount: i32) -> bool {
        let mut status = false;
        let mut sentence = String::new();
        if(!createConnection()) return false;
        
        write!(toserver, "5 {} {} {}\n", fileId, fragmentNum, fragmentCount);
        toserver.flush();

        let re[] = "received!";
        let i = 0;
        for c in inFromServer.chars() {
            if(c == re[i]) i++;
            else return false;
        }

        //status = connect.FileTransporter.sendFile(f, inFromServer, outToServer);
        //没有理解好，还未转换

        if(status) {
            let re[] = "received!";
            let i = 0;
            for c in inFromServer.chars() {
                if(c == re[i]) i++;
                else return false;
            }
        }
    }

    fn createConnection(&mut self) -> bool {
        if(serverIP == null)
            return false;
        let mut toServer = TcpStream::connect(&self.serverIP, &self.serverPort).unwarp();
        toServer.set_nonblocking(true).expect(set nonblocking called failed);

        loop {
            match toServer.read_to_end(&mut self.inFromServer) {
                Ok(_) => break,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    wait_for_fd();
                }
                Err(e) => panic!("encountered IO error: {}", e),
            };
        };
    }

    fn closeConnection(&mut self) {
        self.toServer.close();
        self.inFromServer.close();
    }

}