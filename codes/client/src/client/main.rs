pub mod SynItem;
use std::thread; 
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::{Mutex,Arc};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
fn main() {
    println!("a");
}
let setUpFile = String::from("setup.ini");
pub struct Client{
    pub clientId: i32,
    pub uploadFolders:Vec<String>,
    pub uploadAddrs: Vec<String>,
    pub syn:SynItem::SynItem,
} 

impl Client{
    pub fn new(&self){//初始化结构体
        println!("client start");
    }
    //pub fn getRS()->i32{//返回剩余容量，待实现}
    
    fn setUp(&self) ->bool{ //读取配置文件:每一行为一个数据，并将其存放进一系列变量(内容参见17级详细报告)
        let mut controlPort:i32 = 0;

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
        self.clientId = line.parse::<i32>().unwrap();

        //空行
        fin.read_line(&mut line).unwrap(); 
        
        fin.read_line(&mut line).unwrap(); 
        let mut fragmentFolder = String::from(&line);

        fin.read_line(&mut line).unwrap(); 
        let mut tmpFragmentFolder = String::from(&line);

        fin.read_line(&mut line).unwrap(); 
        let i = line.parse::<i32>().unwrap(); //需监控的上传文件夹数量

        self.uploadFolders = Vec::new();
        self.uploadAddrs = Vec::new();
        let mut j = i;
        while j>0 {
            fin.read_line(&mut line).unwrap(); 
            let uploadFolder = String::from(&line);
            self.uploadFolders.push(uploadFolder);

            fin.read_line(&mut line).unwrap(); 
            let uploadAddr = String::from(&line);
            self.uploadAddrs.push(uploadAddr);
            j-=1;
        }
        /*还要结合其他模块的实现来写调用
        connect.ServerConnecter.init(serverIp, controlPort);
		File file=new File(fragmentFolder);
		if (!file.exists() || !file.isDirectory())
			return false;
		connect.FragmentManager.init(file, serverIp, dataPort);
		file=new File(tmpFragmentFolder);
		if (!file.exists() || !file.isDirectory())
			return false;
		fileDetector.FolderScanner.init(file);
		fileDetector.FileUploader.init(file, serverIp, dataPort);		
        return true;
        */
    }
}

