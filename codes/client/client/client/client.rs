use std::thread; 
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
pub fn main() {
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

    crate::client::connect::ServerConnecter::ServerConnecter::init(&serverIp,&(controlPort as u16));
    let mut file1 = PathBuf::from(&fragmentFolder);
    if !file1.exists() || file1.is_dir(){
        println!("file1 wrong");
        return;
    }

    crate::client::connect::FragmentManager::FragmentManager::init(&file1, &serverIp, &dataPort);
    let mut file2 = PathBuf::from(&tmpFragmentFolder);
    if !file2.exists() || file2.is_dir(){
        println!("file2 wrong");
        return;
    }

    crate::client::fileDetector::FolderScanner::FolderScanner::init(&file2);
    crate::client::fileDetector::FileUploader::FileUploader::init(&file2,&serverIp,&(dataPort as u16)); //note:(by lyf) 类型转换
    

    //线程创建
    let status = Arc::new((Mutex::new(0), Condvar::new()));
    let connect_status = status.clone();
    let fileDetector_status = status.clone();//Arc<Mutex<i32>,Condvar>

    //let clientid = self.clientId.clone();

    let handle1 = thread::spawn(move || { 
        let ServerConnecter = crate::client::connect::ServerConnecter::ServerConnecter::new(clientId);
        ServerConnecter.run(connect_status);
     });//let mut num = counter.lock().unwrap(); *num += 1;
    

    let handle2 = thread::spawn(move || {
    let folderScanner = crate::client::fileDetector::FolderScanner::FolderScanner::new(uploadFolders,uploadAddrs);
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