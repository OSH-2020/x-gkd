#[macro_use]
extern crate mysql;
use std::convert::TryInto;

struct Query{
    pool:mysql::Pool
}
fn main() {
    println!("Hello, world!");
}

impl Query{
    pub fn addFile(&self,file:FileItem) -> i32{
        let suc:i32 = -1;
        if file.isFolder(){
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,true);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,false);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        return suc;
    }

    pub fn deleteFile(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FILE WHERE ID=:id").into_iter() {
            stmt.execute(params!{
                "id" => id
            }).unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn alterFile(&self,file:FileItem) -> i32{
        let suc:i32 = -1;
        if file.isFolder(){
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=true WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa(),
                    "id" => file.getId() 
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=false WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.getName(),
                    "path" => file.getPath(),
                    "attribute" => file.getAttribute(),
                    "time" => file.getTime(),
                    "noa" => file.getNoa(),
                    "id" => file.getId() 
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        suc = 1;
        return suc;
    }

    pub fn alterDevice(&self,device:DeviceItem) -> i32{
        let suc:i32 = -1;
        if device.isOnline(){
            for mut stmt in self.pool.prepare(r"UPDATE DFS.DEVICE SET IP=:ip',PORT=:port,ISONLINE=true,
            RS=:rs WHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.getIp(),
                    "port" => device.getPort(),
                    "rs" => device.getRs(),
                    "id" => device.getId()
                });
            }
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.DEVICE SET IP=:ip,PORT=:port,ISONLINE=false,
            RS=:rsWHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.getIp(),
                    "port" => device.getPort(),
                    "rs" => device.getRs(),
                    "id" => device.getId()
                });   
                suc = match res{
                    Ok(_) => 1,
                    Err(_) => -1
                };
            }
        }       
        return suc;
    }

    pub fn addFragment(&self,id:i32,path:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FRAGMENT VALUES (:id,:path)").into_iter() {
            let res = stmt.execute(params!{
                "id" => id,
                "path" => path
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    } 

    pub fn deleteFragment(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FRAGMENT WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn alterFragment(&self,id:i32,path:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE DFS.FRAGMENT SET PATH=:path WHERE id=:id;").into_iter() {
            let res = stmt.execute(params!{
                "path" => path,
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn addRequest(&self,request:RequestItem) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.REQUEST (TYPE,FRAGMENTID,DEVICEID)
        VALUES (:type,:fragmentid,:deviceid)").into_iter() {
            suc = stmt.execute(params!{
                "type" => request.getType(),
                "fragmentid" => request.getFragementId(),
                "deviceid" => request.getDeviceId()
            }).unwrap().last_insert_id().try_into().unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn deleteRequest(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.REQUEST WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
    pub fn addUser(&self,name:String,passwd:String)-> i32{
        //suc为INSERT的用户在mysql中的id
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.USER (NAME,PASSWD) VALUES (:name, :passwd)").into_iter() {
            suc = stmt.execute(params!{
                "name" => name,
                "passwd" => passwd
            }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功的情况
        }
        suc = 1;
        return suc;
    }

    pub fn alterUser(&self,id:i32,name:String,passwd:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE INTO DFS.USER SET NAEME=:name,PASSWD=passwd WHERE id=:id").into_iter() {
            let res = stmt.execute(params!{
                "name" => name,
                "passwd" => passwd,
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn deleteUser(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.USER WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
}