use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use chrono::Local;
use rand::Rng;
use std::path::PathBuf;
use std::fs::File;

use super::super::database::Query::Query;
use super::super::database::Query::FileItem;
use super::super::database::Query::RequestItem;
use super::FileTransporter;

/*在crate root 中声明 "extern crate chrono;"
cargo.toml中增加：
[dependencies]
chrono = "0.4"
rand = "0.6.0"
*/

//关于null的处理还没有确定，如何调用其他文件中的方法或函数还没有确定
//一部分对null特别处理的代码中，假定变量的类型是Option<T>
//需要参照其他对应文件

pub struct ClientThread{
    client_socket: TcpStream,
    //in_from_server:String,
    //out_to_client:String,
    sentence: String,
    download_folder_path: PathBuf,
    upload_folder_path: PathBuf,
}

impl ClientThread{
    pub fn new(stream:TcpStream)->ClientThread{
        ClientThread{
            client_socket: stream,
            sentence: String::new(),
            download_folder_path: PathBuf::from("/opt/tomcat/webapps/DFS/CloudDriveServer/downloadFragment/"),
            upload_folder_path: PathBuf::from("/opt/tomcat/webapps/DFS/CloudDriveServer/uploadFragment/"),
        }
    }

    pub fn run(&mut self){
        let mut status:bool = false;
        println!("start!");
        //这两行java代码未对应实现
        //clientsocket.setKeepAlive(true);
        //clientsocket.setSoTimeout(5000);
        let in_from_client = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(in_from_client);
        in_from_client.read_line(&mut self.sentence).unwrap();
        println!("D-RECV: {}", self.sentence);
        let command:Vec<&str> = self.sentence[..].split(' ').collect();

        status = match command[0] {
            "1" => self.recv_required_fragment(),
            "2" => self.send_fragment(),
            "3" => self.delete_fragment(),
            "4" => self.register_file(),
            "5" => self.recv_file_fragment(),
            "6" => self.check_folder(),
            _ => {
                self.client_socket.write(b"ERROR!\n");
                self.client_socket.flush();
                false
            },
        };

        if status{
            println!("D-client thread ended (finished)");
        }
        else{
            println!("D-client thread ended (aborted)");
        }
    }

    pub fn recv_required_fragment(&self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();

        let query = Query::new();
        let request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 1{
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let s = self.download_folder_path.into_os_string().into_string().unwrap() + &fid.to_string();
            let recv_file = File::create(s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            status = Filetransporter::recv_file(recv_file, &self.client_socket);
            if status {
                self.client_socket.write(b"received!\n");
                self.client_socket.flush();
                query.deleteRequest(request.get_id());
            }
        }
        //query.closeConnection();
        status
    }

    pub fn send_fragment(&self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();
        
        let query = Query::new();
        let request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 2 {
            status = false;
        }
        else{
            let s = self.upload_folder_path.into_os_string().into_string().unwrap() + &fid.to_string();//upload_folder_path的形式还未确定
            let send_file = File::open(s);
            match send_file{
                Err(e) => {
                    status = false;
                    query.deleteRequest(request.get_id());
                },
                Ok(file) =>{
                    status = super::FileTransporter::send_file(file, &self.client_socket);
                    if status{
                        let mut in_from_cilent = BufReader::new(self.client_socket);
                        let sentence = String::new();
                        in_from_cilent.read_line(&mut sentence).unwrap();
                        match sentence{
                            "received!".to_string() => {
                                //sendFile.delete();
                                query.deleteRequest(request.get_id());
                                //query.alterFragment(fid, Integer.toString(request.getDeviceId()));
                                query.alterFragment(fid, request.get_device_id().to_string());
                            }
                        }
                    }
                }
            };
            
        }
        //query.closeConnection();
        status
    }

    pub fn delete_fragment(&self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();

        let query = Query::new();
        let request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 3 {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            //query.closeConnection();
            status = false;
        }
        else{
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            query.deleteRequest(request.get_id());
            //query.closeConnection();
        }
        status
    }

    pub fn register_file(&self)->bool{
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let noa:i32 = command[5].parse().unwrap();
        let isf:bool = command[6].parse().unwrap();

        let query = Query::new();
        let dt = Local::today();
        let mut date:String = dt.to_string();
        date.truncate(10);
        date.remove(7);
        date.remove(4);
        let fileitem = FileItem::init_2(command[2][..].to_string(), command[3][..].to_string(),
        command[4][..].to_string(), date, -1 * noa, isf);

        let fid = query.addFile(fileitem);
        
        self.client_socket.write_fmt(format_args!("FileId: {}\n", fid));
        self.client_socket.flush();

        //query.closeConnection();
        true
    }

    pub fn recv_file_fragment(&self)->bool{
        let mut status: bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let file_id:i32 = command[1].parse().unwrap();
        let fragment_num:i32 = command[2].parse().unwrap();
        let fragment_count:i32 = command[3].parse().unwrap();

        let query = Query::new();
        let file = query.queryFile_Byid(file_id);

        if file.get_noa() != -1 * fragment_count || fragment_num >= fragment_count || fragment_num < 0 {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let temp = file_id * 100 + fragment_num;
            let s:String = self.upload_folder_path.into_os_string().into_string().unwrap() + &temp.to_string();
            let recv_file = File::create(s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();

            status = FileTransporter::recv_file(recv_file, &self.client_socket);
            if status{
                query.addFragment(temp, "-1".to_string());
                if fragment_num == fragment_count - 1 {
                    let count = query.queryFragmentNumbers(file_id);
                    if count == fragment_count && self.confirm(&file_id, &fragment_count) == 1{
                        self.client_socket.write(b"received!\n");
                        self.client_socket.flush();
                        file.set_noa(fragment_count);
                        query.alterFile(file);
                    }
                    else{
                        self.client_socket.write(b"UPLOADFAIL!\n");
                        self.client_socket.flush();
                        query.deleteFile(file_id);
                        for i in 0..fragment_count{
                            if query.deleteFragment(file_id * 100 + i) == 1 {
                                let temp_2:i32 = file_id * 100 + i;
                                let s:String = self.upload_folder_path.into_os_string().into_string().unwrap() 
                                    + &temp_2.to_string();
                                let f = File::create(s).unwrap();
                            }
                        }
                    }
                }
                else{
                    self.client_socket.write(b"received!\n");
                    self.client_socket.flush();
                }
            }
        }
        //query.closeConnection();
        status
    }

    pub fn check_folder(&self)->bool{
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let num:i32 = command[2].parse().unwrap();

        let query = Query::new();

        let in_from_client = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(in_from_client);
        let mut input = String::new();
        let mut flag: bool = false;
        let mut i = 0;
        for i in 0..num {
            in_from_client.read_line(&mut input).unwrap();
            let ipt = input.as_mut_vec();
            let input_vec:Vec<&str> = input[..].split(' ').collect();
            let file = query.queryFile_Bypathname(Some(ipt[0].to_string()), Some(ipt[1].to_string()));
            if  -1 == file.get_id() {
                let dt = Local::today();
                let mut date:String = dt.to_string();
                date.truncate(10);
                date.remove(7);
                date.remove(4);
                let fileitem = FileItem::init_2(ipt[1].to_string(), ipt[0].to_string(),
                    "rw".to_string(), date, 0, true);
                if query.addFile(file) < 0{
                    flag = true;
                }
            } else {
                if !file.is_folder() {
                    flag = true;
                }
            }
            if flag {
                break;
            }
        }

        if i == num {
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
        }
        else {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
        }

        //query.closeConnection();
        true
    }

    pub fn confirm(&self, id:&i32, num:&i32)->i32{
        let query = Query::new();
        let mut return_val:i32 = 0;

        let di = query.queryOnlineDevice();
        //假定di类型为Vec<DeviceItem>
        if di.is_empty() {
            return -1;
        }

        let s = di.len();
        if num <= &size {
            let t: i32 = rand::thread_rng().gen_range(0, size).try_into().unwrap();
            for i in 0..*num{
                let n: i32 = i.try_into().unwrap();
                let temp = RequestItem::init_2(2, id * 100 + n, di[(n + t) % size].get_id());
                query.addRequest(temp);
            }
        }
        else{
            let mut n:Vec<i32> = Vec::new();
            let temp = num / (size as i32);
            for i in 0..size {
                n.push(temp);
            }
            let m = num % (size as i32);

            let mut t = rand::thread_rng().gen_range(0, size);
            for i in 0..m {
                n[t % size] = n[t % size] + 1;
                t = t + 1;
            }

            let mut k:i32 = 0;
            for i in 0..size {
                for j in 0..n[i] as usize{
                    let temp = RequestItem::init_2(2, id * 100 + (i as i32), di[i].get_id());
                    query.addRequest(temp);
                    k = k + 1;
                }
            }
        }
        return 1
    }
}