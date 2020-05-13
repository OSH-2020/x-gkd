use std::fs::File;
use std::path;
use std::collections::VecDeque;
Struct std::collections::LinkedList;
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

    /*先用 struct std::path::Path 写，没找到如何从 
    struct::fs::File 得到对应 path 的方法*/ 
    fn clearFolder(&self,folder:&path) {
        //原代码中folder是 FILE类型
        if folder.is_file() {
            fs::remove_file(&path);
        } else if folder.is_dir() {
            for entry in fs::read_dir(folder) {
                let entry = entry;
                let path = entry.path();
                if path.is_dir() {
                    clearFolder(&path);
                } else {
                    fs::remove_file(&path);
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

     fn getAllFiles(&self,folder:&path) -> Vec<path>{
         /*!原代码中folder是 FILE类型*/
        let mut files: Vec<path> = Vec::new();

        let queue: LinkedList<path> = LinkedList::new();
        queue.push_back(folder);

        while !queue.is_empty() {
            let dir:path = queue.pop_front;
            for entry in fs::read_dir(dir) {
                let entry = entry;
                let path = entry.path();
                if path.is_dir{
                    queue.push_back(path);
                } else {
                    files.push(path);
                }
            }
        }

        files

}