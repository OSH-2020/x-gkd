use std::path::Path;
use std::fs::metadata;
use std::fs;

use std::path::PathBuf;
use std::{thread, time};
use std::convert::TryInto;
use std::collections::linked_list::LinkedList;

use super::FileUtil::FileUtil;
use crate::client::client::SynItem::SynItem;
use super::FileUploader::FileUploader;
use super::FileAttrs;
use crate::client::com;

use std::sync::{Arc, Mutex, Condvar};

const BYTES_IN_SHARDS:u32 = 500000;
const interval:u32 = 60000;

/* NOTE:
   两个try catch 语句未实现
   文件写权限 writeonly 未实现

*/

/**
 * 定时（默认周期为 2 秒）检测给定的空文件夹<br>
 * 一旦检测到文件放入，检测停止，对加入的文件调用回调接口 FileHandler 的 handle(File file) 方法<br>
 * 所有新加入的文件处理完毕之后，将文件夹清空，继续检测
 */


pub struct FolderScanner{

     folder:Vec<PathBuf>,
     address:Vec<String>,
     synItem:SynItem,

     tmpFragmentFolder:PathBuf,

     // 每次检测的时间间隔
     //interval:u32 = 60000,

     // 是否继续检测的标识，如果为 false 则检测线程停止
     detecting:bool
 }

impl FolderScanner{
     /* 参数syn是client.synItem类型，最后整合时记得改一下*/
     pub fn new(f:Vec<PathBuf>,addr:Vec<String>,syn:SynItem) -> FolderScanner {
         FolderScanner{folder:f,address:addr,synItem:syn,detecting:true,tmpFragmentFolder:PathBuf::new()}
     }
     pub fn init(&self,tmp:PathBuf){
         self.tmpFragmentFolder = tmp;
     }

     //@Override 未实现
     pub fn run(&self,status:Arc<Mutex<i32>,Condvar>){
         let fUploader:FileUploader;
         if !fUploader.checkFolders(self.address){
             println!("ERR: can not register folder");
             //self.synItem.setStatus(2);
             let &(ref lock, ref cvar) = &*status;
             let mut status_cur = lock.lock().unwrap();
             *status_cur = 2;
             cvar.notify_all();
             println!("notify main thread");

             return;
         }
         while self.detecting{
             //未处理catch InterruptedException
            self.scanFiles();
            let interval_mills = time::Duration::from_millis(interval.into());
            thread::sleep(interval_mills);
         }

     }

     // 扫描文件夹，如果有文件加入则处理该文件
     fn scanFiles(&self){
        //let mut i:i32 = 0;
        let FileUtil:FileUtil;
        for i in 0..self.folder.len() {
            let files:LinkedList<PathBuf> = FileUtil.getAllFiles(self.folder[i]);
            for file in files{
                if !self.handleFile(file.as_path().to_path_buf(),i.try_into().unwrap()){
                    return;
                }
            }
            // 处理完毕之后，清空文件夹
			FileUtil.clearFolder(self.folder[i]);
        }
     }

     // 停止检测
     pub fn stopDetecting(&self){
         self.detecting = false;
     }

     pub fn handleFile(&self,file:PathBuf,i:i32) -> bool{
         let fileName:String = file.file_name().unwrap().to_str().unwrap().to_string();
         let filePath:String = self.address[i as usize] + "/";
         
         let mut attribute:String = "".to_string();
         let metadata = file.metadata().unwrap();
         if metadata.permissions().readonly() == true {
             attribute = attribute + "r";
         } else {
             attribute = attribute + "-";
         }
         /* fs::metadata.permissions只在
            全平台实现了readonly(),但writeonly()没实现，
            即下述代码未实现
            if (file.canWrite()) {
                attribute = attribute + 'w';
            } else {
                attribute = attribute + '-';
            }*/
        let mut noa:i32 = (metadata.len().try_into().unwrap() / BYTES_IN_SHARDS) + 1;   //metadata.len()返回值类型为u64
        noa = noa * 2;
        
        let fileAttrs = FileAttrs::FileAttrs::init(fileName,filePath,attribute,noa);
        
        let fUploader:FileUploader;

        let id:i32 = fUploader.registerFile(fileAttrs);
        if id == -2 {
            println!("ERR: can not get file id");
            //self.synItem.setStatus(2);
            let &(ref lock, ref cvar) = &*status;
            let mut status_cur = lock.lock().unwrap();
            *status_cur = 2;
            cvar.notify_all();
            println!("notify main thread");

            return false;
        } else if id == -1 {
            println!("ERR: server already has this file, skip it");
            return true;
        }
        /*NOTE: trycatch 有关erasure code，调用路径可能还需要改*/
        if !com::Encoder::Encoder::encode(file,self.tmpFragmentFolder,id) {
            println!("ERR: can not split file");
            //self.synItem.setStatus(2);
             let &(ref lock, ref cvar) = &*status;
             let mut status_cur = lock.lock().unwrap();
             *status_cur = 2;
             cvar.notify_all();
             println!("notify main thread");

            return false;
        }
        
        for j in 0.. noa {
            if !fUploader.pushFragment(id,j,noa) {
                println!("ERR: can not upload fragments");
                //self.synItem.setStatus(2);
                let &(ref lock, ref cvar) = &*status;
                let mut status_cur = lock.lock().unwrap();
                *status_cur = 2;
                cvar.notify_all();
                println!("notify main thread");

                return false;
            }
        }

        let FileUtil:FileUtil;
        // 处理完毕，清空块文件夹
        FileUtil.clearFolder(self.tmpFragmentFolder);
        
        return true;
     }
 }

