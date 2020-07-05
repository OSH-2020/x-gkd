use std::io::prelude::*;
use std::fs::File;
use std::net::TcpStream;

pub fn recv_file(mut f: File, mut soc_in: &TcpStream)->bool{
    println!("enter FileTransporter -- recv_file");
    //原java文件中socout这个参数并没有用到，此处删去
    //手动实现读取一个long类型的数据
    let mut buffer = [0; 4];
    soc_in.read_exact(&mut buffer).unwrap();
    //Java 数据传输都是big endian，此处也默认读到数据是big endian
    //from_bytes is a nightly-only experimental API.
    //let file_length = i64::from_bytes(buffer);

    
     let file_length:i64 = (((buffer[0] as i64) & 255) << 24) + (((buffer[1] as i64) & 255) << 16)        
     + (((buffer[2] as i64) & 255) << 8) + (((buffer[3] as i64) & 255) << 0);

    let mut toread:i64 = file_length;
    let mut send_bytes = [0; 1024];

    println!("file_length:{}",file_length);


    while toread >= 1024{
        //soc_in.read_exact(&mut send_bytes).unwrap();
        //toread = toread - 1024;
        let readlen = soc_in.read(&mut send_bytes).unwrap();
        toread = toread - readlen as i64;

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
    println!("enter filetransporter--send_file");
    let mut send_bytes = [0; 1024];

    let length = f.metadata().unwrap().len();

    //soc_out.write(b(format!("{:08}", length)));
    soc_out.flush();

    loop {
        let readlen = f.read(&mut send_bytes[..]);
        let len: i32 = match readlen{
            Err(e) => -1,
            Ok(len) => len as i32,
        };
        if len == -1 {
            return false;
        }
        if len == 0 {
            break;
        }
        soc_out.write(&mut send_bytes[..]);
        soc_out.flush();
       
    }

    return true
}