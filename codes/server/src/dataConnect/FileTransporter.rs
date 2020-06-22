use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;

pub fn recv_file(mut f: File, mut soc_in: &TcpStream)->bool{
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
    
    let mut toread:i64 = file_length;
    let mut send_bytes = [0; 1024];
    while toread >= 1024{
        soc_in.read_exact(&mut send_bytes).unwrap();
        toread = toread - 1024;
        f.write(&send_bytes);
        f.flush();
    }
    let mut file_end: Vec<u8> = Vec::new();
    soc_in.read_to_end(&mut file_end).unwrap();
    f.write(&file_end);
    f.flush();
    //没有再创建FileOutputStream对象，这里不需要关闭什么
    return true
}//TODO:err handle

pub fn send_file(mut f: File, mut soc_out: &TcpStream)->bool{
    let mut send_bytes = [0; 4096];

    let mut length = f.read(&mut send_bytes[..]);
    let length = match length{
        Ok(len) => len as i64,
        Err(error) => {
            println!("can't read file :{:?}", error);
            return false},
    };
    //println!("read:{}", length);
    let bytes = length.to_be_bytes();
    soc_out.write(&bytes);//TODO:
    soc_out.flush();
    let n2 = soc_out.write(&mut send_bytes[..]);
    let n2 = match n2{
        Ok(n) => n as i32,
        Err(error) => panic!("Problem write file: {:?}", error),
    };
    soc_out.flush();
    return true
}