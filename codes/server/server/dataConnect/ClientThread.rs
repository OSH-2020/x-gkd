use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use chrono::Local;
use rand::Rng;
use std::path::Path;

/*在crate root 中声明 "extern crate chrono;"
cargo.toml中增加：
[dependencies]
chrono = "0.4"
rand = "0.6.0"
*/

//关于null的处理还没有确定，如何调用其他文件中的方法或函数还没有确定
//一部分对null特别处理的代码中，假定变量的类型是Option<T>
//需要参照其他对应文件

struct ClientThread{
    client_socket:TcpStream,
    //in_from_server:String,
    //out_to_client:String,
    sentence:String,
    download_folder_path:Path,
    upload_folder_path:Path,
}

impl ClientThread{
    pub fn new(stream:TcpStream)->ClientThread{
        ClientThread{
            client_socket:stream,
            sentence:String::new(),
            download_folder_path:Path::new("/opt/tomcat/webapps/DFS/CloudDriveServer/downloadFragment/");
            upload_folder_path:Path::new("/opt/tomcat/webapps/DFS/CloudDriveServer/uploadFragment/");
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

        let query = super::super::database::Query::new();
        let request = query.queryRequest_Byid(id);

        if request.getFragmentId() != fid || request.getType() != 1{
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let s = self.download_folder_path.to_string() + fid.to_string();
            let recv_file = File::create(s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            status = recv_file(recv_file, &self.client_socket);
            if status {
                self.client_socket.write(b"received!\n");
                self.client_socket.flush();
                query.deleteRequest(request.getId());
            }
        }
        query.closeConnection();
        status
    }

    pub fn send_fragment(&self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();
        
        let query = super::super::database::Query::new();
        let request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.getType() != 2 {
            status = false;
        }
        else{
            let s = self.upload_folder_path.to_string() + fid.to_string();//upload_folder_path的形式还未确定
            let send_file = File::open(s);
            match send_file{
                Err => {
                    status = false;
                    query.daleteRequest(request.getId());
                },
                Ok(file) =>{
                    status = super::FileTransporter::send_file(file, self.client_socket);
                    if status{
                        let mut in_from_cilent = BufReader::new(client_socket);
                        let sentence = String::new();
                        in_from_cilent.read_line(&mut sentence).unwrap();
                        match sentence{
                            "received!" => {
                                //sendFile.delete();
                                query.deleteRequest(request.get_id());
						        query.alterFragment(fid, Integer.toString(request.getDeviceId()));
                            }
                        }
                    }
                }
            };
            
        }
        query.closeConnection();
        status
    }

    pub fn delete_fragment(&self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();

        let query = super::super::database::Query::new();
        let request = query.queryRequest_Byid(id);

        if request.getFragmentId() != fid || request.getType() != 3 {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            query.closeConnection();
            status = false;
        }
        else{
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            query.deleteRequest(request.get_id());
            query.closeConnection();
        }
        status
    }

    pub fn register_file(&self)->bool{
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let noa:i32 = command[5].parse().unwrap();
        let isf:bool = command[6].parse().unwrap();

        let query = super::super::database::Query::new();
        let dt = Local::today();
        let mut date:String = dt.to_string();
        date.truncate(10);
        date.remove(7);
        date.remove(4);
        let fileitem = super::super::database::fileitem::init_2(command[2][..], command[3][..],
        command[4][..], date, -1 * noa, isf);

        int fid = query.addFile(fileitem);
        
        self.client_socket.write_fmt(format_args!("FileId: {}\n", fid));
        self.client_socket.flush();

        query.closeConnection();
        true
    }

    pub fn recv_file_fragment(&self)->bool{
        let mut status: bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let file_id:i32 = command[1].parse().unwrap();
        let fragment_num:i32 = command[2].parse().unwrap();
        let fragment_count:i32 = command[3].parse().unwrap();

        let query = super::super::database::Query::new();
        let file = query.queryFile_Byid(file_id);

        if (file.getNoa() != -1 * fragment_count || fragment_num >= fragment_count || fragment_num < 0){
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let temp = file_id * 100 + fragment_num;
            let s:String = self.upload_folder_path.to_string() + temp.to_string();
            let recv_file = File::create(s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();

            status = super::FileTransporter::recv_file(recv_file, &self.client_socket);
            if status{
                query.addFragment(temp, "-1");
                if fragment_num == fragment_count - 1 {
                    let count = query.queryFragmentNumbers(file_id);
                    if count == fragment_count && self.confirm(file_id, fragment_count) == 1{
                        self.client_socket.write(b"received!\n");
                        self.client_socket.flush();
                        file.setNoa(fragment_count);
                        query.alterFile(file);
                    }
                    else{
                        self.client_socket.write(b"UPLOADFAIL!\n");
                        self.client_socket.flush();
                        query.deleteFile(file_id);
                        for i in 0..fragment_count{
                            if query.deleteFragment(file_id * 100 + i) == 1 {
                                let temp_2:i32 = file_id * 100 + i;
                                let s:String = self.upload_folder_path.to_string() + temp_2.to_string();
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
        query.closeConnection();
        status
    }

    pub fn check_folder(&self)->bool{
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let num:i32 = command[2].parse().unwrap();

        let query = super::super::database::Query::new();

        let in_from_client = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(in_from_client);
        let mut input = String::new();
        let mut flag:bool = false;
        for i in 0..num {
            in_from_client.read_line(&mut input).unwrap();
            let input_vec:Vec<&str> = input[..].split(' ').collect();
            let file = query.queryFile_Bypathname(input[0].to_string(), input[1].to_string());
            match file{
                None => {
                    let dt = Local::today();
                    let mut date:String = dt.to_string();
                    date.truncate(10);
                    date.remove(7);
                    date.remove(4);
                    let fileitem = fileitem::init_2(input[1][..], input[0][..],
                    "rw", date, 0, true);
                    if query.addFile(file) < 0{
                        flag = true;
                    }
                },
                Some(file){
                    if !file.is_folder() {
                        flag = true;
                    }
                },
            };
            if (flag){
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

        query.closeConnection();
        true
    }

    pub fn confirm(id:&i32, num:&i32)->i32{
        let query = super::super::database::Query::new();
        let mut return_val:i32 = 0;

        let di = query.queryOnlineDevice();
        //假定di类型为Vec<DeviceItem>
        if di.is_empty() {
            return -1;
        }

        let size = di.len();
        if num <= size {
            let t = rand::thread_rng().gen_range(0, size);
            for i in 0..num{
                let temp = super::super::database::RequestItem::init_2(2, id * 100 + i, di[(i + t) % size].getId())
                query.addRequest(temp);
            }
        }
        else{
            let mut n:Vec<i32> = Vec::new();
            let temp = num / size;
            for i in 0..size {
                n.push(temp);
            }
            let m = num % size;

            let mut t = rand::thread_rng().gen_range(0, size);
            for i in 0..m {
                n[t % size] = n[t % size] + 1;
                t = t + 1;
            }

            let mut k:i32 = 0;
            for i in 0..size {
                for j in 0..n[i] as usize{
                    let temp = database::RequestItem::init_2(2, id * 100 + i, di[i].getId());
                    query.addRequest(temp);
                    k = k + 1;
                }
            }
        }
        return 1
    }
}