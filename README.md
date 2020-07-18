# x-gkd

### OSH2020大作业

***

#### **题目：**

**基于 Rust 和 WebAssembly 的互联网分布式文件系统**



#### 成员

**雷雨轩**

**裴启智**

**刘逸菲**

**曲阳**

**孙一鸣**



#### 项目结构

1. codes
   - client
     - src
       - client
         - client
           - client.rs
           - SynItem.rs
         - com
           - Decoder.rs
           - Encoder.rs
         - connect
           - FileTransporter.rs
           - FragmentManager.rs
           - ServerConnecter.rs
         - fileDetector
           - FileAttr.rs
           - FileUploader.rs
           - FileUtil.rs
           - FolderScanner.rs
     - Cargo.toml
   - server
     - src
       - server
         - controlConnect
           - ClientThread.rs
           - ServerThread.rs
         - database
           - DeviceItem.rs
           - FileItem.rs
           - Query.rs
           - RequestItem.rs
         - dataConnect
           - ClientThread.rs
           - FileTransporter.rs
           - ServerThread.rs
     - Cargo.toml
   - web
     - backend
       - src
         - com
           - Decoder.rs
           - Encoder.rs
         - database
           - DeviceItem.rs
           - FileItem.rs
           - Query.rs
           - RequestItem.rs
         - userManagement
           - FileDownloader.rs
           - GetFileList.rs
           - UserLogin.rs
           - UserReg.rs
       - Cargo.toml
     - frontend
     - seed-frontend
2. discussion
3. docs
   - 调研报告
   - 可行性报告
   - 中期报告
   - 详细设计报告
   - 部署说明文档
4. temp
   - records
   - web 端调研
   - 可行性调研
   - 中期报告整理