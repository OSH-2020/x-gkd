use std::thread;
use std::path::Path;
use std::fs::metadata;
/* NOTE:
   两个try catch 语句未实现
   文件写权限 writeonly 未实现

*/

/**
 * 定时（默认周期为 2 秒）检测给定的空文件夹<br>
 * 一旦检测到文件放入，检测停止，对加入的文件调用回调接口 FileHandler 的 handle(File file) 方法<br>
 * 所有新加入的文件处理完毕之后，将文件夹清空，继续检测
 */

 struct FolderScanner{
     pub BYTES_IN_SHARDS:u32 = 500000,

     folder:Vec<Path>,
     address:Vec<String>,
     synItem:clent.SynItem,

     tmpFragmentFolder:Path,

     // 每次检测的时间间隔
     interval:u32 = 60000,

     // 是否继续检测的标识，如果为 false 则检测线程停止
     detecting:boolean = true,
 }

 impl FolderScanner{
     pub fn new(f:vec<Path>,addr:Vec<String>,syn:client.synItem){
         FolderScanner{fold:f,address:adr,synItem:syn}
     }
     pub fn init(&self,tmp:Path){
         tmpFragmentFolder = tmp;
     }
     pub fn run(&self){
         fUploader:FileUploader;
         if !fUploader.checkFolders(address){
             println!("ERR: can not register folder");
             synItem.setStatus(2);
             return;
         }
         while detecting{
             //!try catch
         }

     }

     // 扫描文件夹，如果有文件加入则处理该文件
     fn scanFiles(){

        for i:i32 in 0..folder.len {
            files:Vec<Path> = FileUtil.getAllFiles(folder[i]);
            for file in files{
                if(!handlerFile(file,i));
                    return;
            }
            // 处理完毕之后，清空文件夹
			FileUtil.clearFolder(folder[i]);
        }
     }

     // 停止检测
     pub fn stopDetecting(){
         detecting = false;
     }

     pub fn handleFile(file:Path,i:i32) -> boolean{
         let fileName:String = file.file_name;
         let filePath:String = file.to_str + '/';
         /*let mut s1 = "Hello,".to_string();
let s2 = "world".to_string();
s1 += &s2;*/
         let attribut::String = "";
         let metadata = fs::metadata(&file);
         if metadata.permissions.readonly() == true {
             attribute = attribute + 'r';
         } else {
             attribute = attribute + '-';
         }
         /* fs::metadata.permissions只在
            全平台实现了readonly(),但writeonly()没实现，
            即下述代码未实现
            if (file.canWrite()) {
                attribute = attribute + 'w';
            } else {
                attribute = attribute + '-';
            }*/
        let mut noa:i32 = (i32)(metadata.len() /BYTES_IN_SHARDS) + 1;   //metadata.len()返回值类型为u64
        noa = noa * 2;
        
        let fileAttrs = FileAttrs::new(fileName,filePath,attribute,noa);
        
        let fuploader:FileUploader;

        id:i32 = fUploader.registerFile(fileAttrs);
        if id == -2 {
            println!("ERR: can not get file id");
            synItem:setStatus(2);
            return false;
        } else if id == -1 {
            println!("ERR: server already has this file, skip it");
            return true;
        }
        //trycatch

        for j:i32 in 0.. noa {
            if(!fUploader.pushFragment(id,j,noa)){
                println!("ERR: can not upload fragments");
                synItem.setStatus(2);
                return false;
            }
        }

        // 处理完毕，清空块文件夹
        FileUtil.clearFolder(tmpFragmentFolder);
        
        return true;
     }
 }