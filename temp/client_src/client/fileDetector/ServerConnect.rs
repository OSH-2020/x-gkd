use std::net::{TcpStream, Ipv4Addr, Shutdown};
use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use std::{thread, time};

// //
use super::FragmentManager;
// //

// //#[derive(Debug)]
// struct ServerConnecter{
//     server_ip:String,
//     control_port:u16,
//     client_id:i32,
//     connecting:bool,
//     //private client.SynItem syn;
//     to_server:Option<TcpStream>,
// }

// impl ServerConnecter{

//     pub fn new(c_id:i32)->ServerConnecter{//client.SynItem s
//         ServerConnecter{
//             server_ip: String::new(),
//             control_port: 0,
//             client_id: c_id,
//             connecting: false,
//             to_server:None,
//         }
//     }

//     pub fn init(&mut self, s_ip:String, c_port:u16){
//         self.server_ip = s_ip.clone();
//         self.control_port = c_port;
//     }

//     pub fn run(&mut self){

//         let mut status = true;

//         while self.connecting{
//             if let Ok(connect_socket) = TcpStream::connect((&self.server_ip[..], self.control_port)) {
//                 self.to_server = Some(connect_socket);
//                 println!("Connect to server successfully(control)!");
//             } else {
//                 println!("Couldn't connect to server...");
//                 status = false;
//             }
            
//             if !status{
//                 break;
//             }

//             let mut input_buf = String::new();
//             match &mut self.to_server{
//                 None => println!("Error! server not connected..."),
//                 Some (socket) => {
//                     let socket_read = socket.try_clone().expect("clone failed...");
//                     let mut in_from_server = BufReader::new(socket_read);
//                     while self.connecting{
//                         //我不知道原文件的client.Client.getRS()是什么东西所以没有写
//                         socket.write_fmt(format_args!("1 {}\n", self.client_id.to_string()));//TODO:err handle
//                         socket.flush();//TODO:err
//                         in_from_server.read_line(&mut input_buf).unwrap();
//                         let input_buf = input_buf.trim();
//                         let mut input_vec:Vec<&str>= input_buf[..].split(' ').collect();

//                         //debug
//                         println!("input is: {}", input_buf);

//                         let mut unread_request:u32 = input_vec[2].parse().unwrap();
                        
//                         let mut inputline = String::new();
//                         while unread_request>0 {
//                             socket.write_fmt(format_args!("2 {}\n", self.client_id.to_string()));
//                             socket.flush();
//                             in_from_server.read_line(&mut inputline).unwrap();
//                             let inputline = inputline.trim();
//                             let mut input_vec:Vec<&str>= inputline[..].split(' ').collect();
//                             let request_id:u32 = input_vec[0].parse().unwrap();
//                             let fragment_id:u32 = input_vec[1].parse().unwrap();
//                             let ftype:u32 = input_vec[2].parse().unwrap();
//                             //以下两行用到其他文件中定义的结构体
//                             let f_manager = FragmentManager::new(request_id, fragment_id, ftype);
//                             f_manager.submit();
//                             unread_request = unread_request - 1;
//                         }
//                     }
                
//                 }
//             }
//             //sleep
//             let five_seconds = time::Duration::new(5, 0);
//             thread::sleep(five_seconds);

//             match &mut self.to_server{
//                 None => println!("Error! server not connected..."),
//                 Some (socket) => {
//                     socket.write(b"exit\n");
//                     socket.flush();
//                     socket.shutdown(Shutdown::Both)
//                         .expect("socket shutdown call failed");
//                 }
//             }
//         }
//         if self.connecting {
//             //syn.setStatus(1);
//             println!("ERR: connect to server has been interrupted!");
//         }
//     }

//     pub fn stopConnect(&mut self){
//         self.connecting = false;
//     }

// }