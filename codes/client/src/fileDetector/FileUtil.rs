//use std::fs::File;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::collections::LinkedList;
use std::vec::Vec;
//use std::convert::From::from;

fn main() {

    /**
     * 文件工具类
     */
    struct FileUtil{
    }
    
    impl FileUtil{
    
        /**
         * 清空文件夹
         *
         * @param folderPath 文件夹路径
         */
    
        /*由于直接用了path写，所以这段没改写
        public static void clearFolder(String folderPath) {
            clearFolder(new File(folderPath));
        }*/
        // fn clearFolder_str(&self,folderPath:String) {
        //     self.clearFolder(&(*std::convert::From::from(folderPath)));
        // }
    
        /*先用 struct std::path::Path 写，没找到如何从 
        struct::fs::File 得到对应 path 的方法*/ 
        pub fn clearFolder(&self,folder:&Path) {
            //原代码中folder是 FILE类型
            if folder.is_file() {
                fs::remove_file(&folder);
            } else if folder.is_dir() {
                if let Ok(entries) = fs::read_dir(folder){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(metadata) = entry.metadata(){
                            let pathbuf = entry.path();
                            let path:&Path = pathbuf.as_path();
                            if path.is_dir() {
                                self.clearFolder(path);
                            } else {
                                fs::remove_file(path);
                            }
                        }
                    
                    // let path = match entry{
                    //     Ok(_) => entry.path()
                    // };
                    
                    }
                }
               }
            }
            
        }
        /**
         * 广度优先遍历文件夹及其子文件夹，获得该文件夹下所有的文件
         *
         * @param folder 顶层文件夹
         * @return 所有的文件
         */
    
         fn getAllFiles(&self,folder:&Path) -> LinkedList<File>{
             /*!原代码中folder是 FILE类型*/
            
            let mut files:LinkedList<File> = LinkedList::new();
            let mut queue: LinkedList<PathBuf> = LinkedList::new();          

            queue.push_back(folder.to_path_buf());
    
            while !queue.is_empty() {
                let dir:PathBuf = queue.pop_front().unwrap();
                
                if let Ok(entries) = fs::read_dir(dir){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(metadata) = entry.metadata(){
                            
                            let pathbuf = entry.path(); //.path() -> pathbuf类型
                            if pathbuf.is_dir(){
                                queue.push_back(pathbuf);
                            } else {
                                files.push_back(File::open(pathbuf).unwrap());
                            }
                        }
                    
                    }
                }
                }
            }
            files
    
        }
    }
}