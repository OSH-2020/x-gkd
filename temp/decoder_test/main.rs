use std::*;
pub mod Decoder;
use std::string::String;
use std::path::PathBuf;
use std::vec;


fn main() {
    println!("decodeFile is called");

    //let path: String = String::from("TIM/");
    //let name: String = String::from("b.txt");
    //let query = Query::new();
    //let fileItem = query.queryFile_Bypathname(Some(&path), Some(&name));
    //let onlineDevice: Vec<DeviceItem> = query.queryOnlineDevice();

    /*if onlineDevice[0] == 0 {
        println!("NotEnoughFragments");
    }*/

    let fragmentFolderPath: PathBuf = PathBuf::from("D:/webapps/DFS/CloudDriveServer/downloadFragment");
    let fileFolderPath: PathBuf = PathBuf::from("D:/webapps/DFS/CloudDriveServer/tmpFile/b.txt");
    //fileFolderPath.push(&name);

    //if fileItem[0] == 0 {
        //println!("Error");
        //return "success".to_string();
    //} else {
        Decoder::Decoder::decode(fragmentFolderPath, fileFolderPath,
            2, 2); /*{
                let file_id = fileItem.get_id();
                let fileId = file_id.to_string();
                if let Ok(entries) = fs::read_dir(fragmentFolderPath){
                    for entry in entries{
                        if let Ok(entry) = entry{
                            let pathbuf = entry.path();
                            let str: String = pathbuf.file_name().unwrap().to_os_string().into_string().unwrap();
                            str.shrink_to(str.len()-2);
                            if str.eq(fileId) {
                                let path:&Path = pathbuf.as_path();
                                fs::remove_file(path).unwrap();
                            }
                        }
                    }
                }
                {
                    println!()
                }
            }*/
    //}
}
