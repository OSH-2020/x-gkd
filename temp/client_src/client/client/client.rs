// //pub mod SynItem;
use std::thread; 
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
fn main() {
    let mut clientId:i32 = 0;
    let mut uploadFolders:Vec<PathBuf> = Vec::new();
    let mut uploadAddrs:Vec<String> = Vec::new();
    println!("client start");


    //read setup.ini 
    let mut controlPort:i32 = 0;
        
    let setUpFile = String::from("setup.ini");
    let file = File::open(setUpFile).unwrap();
    let mut fin = BufReader::new(file);
    let mut line = String::new();

    fin.read_line(&mut line).unwrap(); 
    let mut serverIp = String::from(&line);

    fin.read_line(&mut line).unwrap(); 
    let mut controlPort = line.parse::<i32>().unwrap();
    
    fin.read_line(&mut line).unwrap(); 
    let mut dataPort = line.parse::<i32>().unwrap();

    fin.read_line(&mut line).unwrap(); 
    clientId = line.parse::<i32>().unwrap();

    //空行
    fin.read_line(&mut line).unwrap(); 
    
    fin.read_line(&mut line).unwrap(); 
    let mut fragmentFolder = String::from(&line);

    fin.read_line(&mut line).unwrap(); 
    let mut tmpFragmentFolder = String::from(&line);

    fin.read_line(&mut line).unwrap(); 
    let i = line.parse::<i32>().unwrap(); //需监控的上传文件夹数量

    //self.uploadFolders = Vec::new();
    //self.uploadAddrs = Vec::new();
    let mut j = i;
    while j>0 {
        fin.read_line(&mut line).unwrap(); 
        let uploadFolder = PathBuf::from(&line);
        uploadFolders.push(uploadFolder);

        fin.read_line(&mut line).unwrap(); 
        let uploadAddr = String::from(&line);
        uploadAddrs.push(uploadAddr);
        j-=1;
    }

    crate::client::connect::ServerConnecter::init(&serverIp,&controlPort);
    let mut file1 = PathBuf::from(&fragmentFolder);
    if !file1.exists() || file1.is_dir(){
        println!("file1 wrong");
        return;
    }

    crate::client::connect::FragmentManager::init(&file1, &serverIp, &dataPort);
    let mut file2 = PathBuf::from(&tmpFragmentFolder);
    if !file2.exists() || file2.is_dir(){
        println!("file2 wrong");
        return;
    }

    fileDetector::FolderScanner::init(&file2);
    fileDetector::FileUploader::init(&file2,&serverIp,&dataPort);
    

    //线程创建
    let status = Arc::new((Mutex::new(0), Condvar::new()));
    let connect_status = status.clone();
    let fileDetector_status = status.clone();//Arc<Mutex<i32>,Condvar>

    //let clientid = self.clientId.clone();

    let handle1 = thread::spawn(move || { 
        let ServerConnecter = crate::client::connect::ServerConnecter::new(clientId);
        ServerConnecter.run(connect_status);
     });//let mut num = counter.lock().unwrap(); *num += 1;
    

    let handle2 = thread::spawn(move || {
    let folderScanner = crate::client::fileDetector::FolderScanner::new(uploadFolders,uploadAddrs);
    folderScanner.run(fileDetector_status);
    });

    let &(ref lock, ref cvar) = &*status;
    let mut status_cur = lock.lock().unwrap();
    while *status_cur==0 {//状态码未被改变时，则继续wait
        println!("before wait");
        status_cur = cvar.wait(status_cur).unwrap();
        println!("after wait");
    }
    
    if *status_cur==1 {
        println!("Err: can not connect to server");
    }else if *status_cur==2{
        println!("Err: can detect files");
    }
}

/*
use std::thread; 
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::{Mutex,Arc};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
pub fn main() {
     println!("a");
     crate::client::connect::ServerConnect::init();
     super::SynItem::waitChange();
         }
*/
// //let setUpFile = String::from("setup.ini");
// pub struct Client{
//     pub clientId: i32,
//     pub uploadFolders:Vec<String>,
//     pub uploadAddrs: Vec<String>,
//     pub syn:SynItem::SynItem,
// } 

// impl Client{
//     pub fn new(&self){//初始化结构体
//         println!("client start");
//     }
//     //pub fn getRS()->i32{//返回剩余容量，待实现}
    
//     fn setUp(&self) ->bool{ //读取配置文件:每一行为一个数据，并将其存放进一系列变量(内容参见17级详细报告)
//         let mut controlPort:i32 = 0;

//         let file = File::open(setUpFile).unwrap();
//         let mut fin = BufReader::new(file);
//         let mut line = String::new();

//         fin.read_line(&mut line).unwrap(); 
//         let mut serverIp = String::from(&line);

//         fin.read_line(&mut line).unwrap(); 
//         let mut controlPort = line.parse::<i32>().unwrap();
        
//         fin.read_line(&mut line).unwrap(); 
//         let mut dataPort = line.parse::<i32>().unwrap();

//         fin.read_line(&mut line).unwrap(); 
//         self.clientId = line.parse::<i32>().unwrap();

//         //空行
//         fin.read_line(&mut line).unwrap(); 
        
//         fin.read_line(&mut line).unwrap(); 
//         let mut fragmentFolder = String::from(&line);

//         fin.read_line(&mut line).unwrap(); 
//         let mut tmpFragmentFolder = String::from(&line);

//         fin.read_line(&mut line).unwrap(); 
//         let i = line.parse::<i32>().unwrap(); //需监控的上传文件夹数量

//         self.uploadFolders = Vec::new();
//         self.uploadAddrs = Vec::new();
//         let mut j = i;
//         while j>0 {
//             fin.read_line(&mut line).unwrap(); 
//             let uploadFolder = String::from(&line);
//             self.uploadFolders.push(uploadFolder);

//             fin.read_line(&mut line).unwrap(); 
//             let uploadAddr = String::from(&line);
//             self.uploadAddrs.push(uploadAddr);
//             j-=1;
//         }
//         /*还要结合其他模块的实现来写调用
//         connect.ServerConnecter.init(serverIp, controlPort);
// 		File file=new File(fragmentFolder);
// 		if (!file.exists() || !file.isDirectory())
// 			return false;
// 		connect.FragmentManager.init(file, serverIp, dataPort);
// 		file=new File(tmpFragmentFolder);
// 		if (!file.exists() || !file.isDirectory())
// 			return false;
// 		fileDetector.FolderScanner.init(file);
// 		fileDetector.FileUploader.init(file, serverIp, dataPort);		
//         return true;
//         */
//     }
// }

