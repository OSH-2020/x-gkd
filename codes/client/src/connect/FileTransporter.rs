use std::io::{Stdin, Stdout};
use std::io::prelude::*;
use std::fs::File;

pub fn send_file(mut f:File)->bool{
    //将文件全部发送到stdout

    let mut send_bytes = [0; 1024];

    let mut length = f.read(&mut send_bytes[..]);
    let length = match length{
        Ok(len) => len as i64,
        Err(error) => {
            println!("can't read file :{:?}", error);
            return false},
    };
    //println!("read:{}", length);
    let bytes = length.to_be_bytes();
    std::io::stdout().write(&bytes);//TODO:
    std::io::stdout().flush();
    let n2 = std::io::stdout().write(&mut send_bytes[..]);
    let n2 = match n2{
        Ok(n) => n as i32,
        Err(error) => panic!("Problem write file: {:?}", error),
    };
    std::io::stdout().flush();
    return true
}

pub fn recv_file(mut f:File, mut soc_in:Stdin)->bool{
    //原java文件中socout这个参数并没有用到，此处删去
    //手动实现读取一个long类型的数据
    let mut buffer = [0; 8];
    soc_in.read_exact(&mut buffer).unwrap();
    //Java 数据传输都是big endian，此处也默认读到数据是big endian
    //from_bytes is a nightly-only experimental API.
    //let file_length = i64::from_bytes(buffer);
    let file_length:i64 = ((buffer[0] as i64) << 56) + (((buffer[1] as i64) & 255) << 48) + (((buffer[2] as i64) & 255) << 40)       
     + (((buffer[3] as i64) & 255) << 32) + (((buffer[4] as i64) & 255) << 24) + (((buffer[5] as i64) & 255) << 16)        
     + (((buffer[6] as i64) & 255) << 8) + (((buffer[7] as i64) & 255) << 0);
    
    let mut read:i64 = 0;
    let mut r:i64 = 0;
    let mut send_bytes = [0; 1024];
    while read < file_length{
        let read_res = soc_in.read(&mut send_bytes[..]);
        let r = match read_res{
            Ok(read_once) => read_once as i64,
            Err(error) => panic!("Problem receieve file: {:?}", error),
        };
        read = read + r;
        f.write(&send_bytes);//写入的数据量不能控制
        f.flush();
    }
    //没有再创建FileOutputStream对象，这里不需要关闭什么
    return true
}//TODO:err handle