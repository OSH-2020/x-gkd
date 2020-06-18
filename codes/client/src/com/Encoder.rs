#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;
use std::fs::File;
use std::path;
use std::fs;
use std::path::Path;
use std::convert::TryInto;
use std::io::Read;

const BYTES_IN_SHARDS:i32 = 500000;
const BYTES_IN_INT :i32 = 4;

struct Encoder{
}

impl Encoder{
    /*原参数的FILE 改为 Path，未实现 throw IOEception */
    fn encode(&self,inputFile_Path:&Path,shardsFolder:&Path,fid:i32) -> bool{
        if !inputFile_Path.exists() {
            //注：std::path::Path的exists()方法，由于权限错误而无法访问包含文件的目录，也将返回false。
            println!("Cannot read input file: {}" ,inputFile_Path.display());
            return false;
        }

        // Get the size of the input file.  (Files bigger that
        // Integer.MAX_VALUE will fail here!)
        //  注：原Java代码中使用long java.io.File.length()得到filesize，但当inputFile为
        //      目录时，返回值（即filesize）是unspecified
        let mut inputFile:File = File::open(&inputFile_Path).unwrap();
        let fileSize:i32 = inputFile.metadata().unwrap().len().try_into().unwrap(); //len()返回u64
        let dataShards:i32 = fileSize / BYTES_IN_SHARDS + 1;
        let totalShards:i32 = 2 * dataShards;

        // Figure out how big each shard will be.  The total size stored
        // will be the file size (8 bytes) plus the file.
        let storedSize:i32 = fileSize + BYTES_IN_INT;
        let shardSize:i32 = (storedSize + dataShards - 1) / dataShards;

        // Create a buffer holding the file size, followed by
        // the contents of the file.
        let bufferSize:i32 = shardSize * dataShards;
        
        let mut allBytes:Vec<u8> = Vec::new();
        //注：原程序中是声明了一个长度为bufferSize的数组
        
        //let mut allBytes:[u8;bufferSize]; rust中不能用变量声明数组长度
        //allBytes.push(fileSize.to_be_bytes()); 
        for temp in fileSize.to_be_bytes().iter(){
            //to_be_bytes()大尾顺序
            allBytes.push(*temp);
        }
        inputFile.read_to_end(&mut allBytes);   //appended,故不需要指定off

        if allBytes.len() != (fileSize + 4).try_into().unwrap() {
            panic!("not enough bytes read");
            //注：原程序为throw IOException
        }

        if allBytes.len() > bufferSize.try_into().unwrap() {
            allBytes.resize(bufferSize.try_into().unwrap(),0);
        }
        
        //let mut shards = shards!(allBytes);

        // let mut i:u32 = 0;
        // let mut j:u32 = 0;
        //byte[totalShards][shardSize]
        let mut shards:Vec<Vec<u8>> = vec![vec![0;shardSize.try_into().unwrap()];totalShards.try_into().unwrap()];
        
        //let index:Vec<usize> = vec![]
        for i in 0..dataShards as usize {
            for j in 0..shardSize.try_into().unwrap(){
                &shards[i].push(allBytes[((i as u32) * (shardSize as u32) + j) as usize]);
            }
        }

        // for i in (dataShards+1)..totalShards.try_into().unwrap() {
        //     for j in 0..shardSize.try_into().unwrap() {
        //         &shards[i].push(0);
        //     }
        // }

        // Use Reed-Solomon to calculate the parity.
        let reedSolomon = ReedSolomon::new(dataShards.try_into().unwrap(),(totalShards - dataShards).try_into().unwrap()).unwrap();
        reedSolomon.encode(&mut shards).unwrap();

        //let mut shards:Vec<_> = shards.iter().cloned().map(Some).collect();
        
        // Write out the resulting files.
        for i in 0..totalShards.try_into().unwrap() {
            let pathbuf = shardsFolder.join(Path::new(&(fid * 100 + i as i32).to_string()));
            let path:&Path = pathbuf.as_path();
            fs::write(&path,&shards[i]).unwrap();
        }

        println!("Encode Success");
        return true;
    }
}