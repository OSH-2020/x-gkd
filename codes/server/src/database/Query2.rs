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
        return suc;
    }

    pub fn deleteRequest(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.REQUEST WHERE ID=:id").into_iter() {
            stmt.execute(params!{
                "id" => id
            }).unwrap();
        }
        //此处未处理execute不成功时，返回-1的情况
        return 1;
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
        return suc;
    }

    pub fn alterUser(&self,id:i32,name:String,passwd:String) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE INTO DFS.USER SET NAEME=:name,PASSWD=passwd WHERE id=:id").into_iter() {
            stmt.execute(params!{
                "name" => name,
                "passwd" => passwd,
                "id" => id
            }).unwrap();
            //此处未处理execute不成功时，返回-1的情况
        }
        return 1;
    }

    pub fn deleteUser(&self,id:i32) -> i32{
        let suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.USER WHERE ID=:id").into_iter() {
            stmt.execute(params!{
                "id" => id
            }).unwrap();
        }
        //此处未处理execute不成功时，返回-1的情况
        return 1;

    }
}