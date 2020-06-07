use mysql::*;
use mysql::preclude::*;
pub mod FileItem;

pub struct Query {
    pool: MyPool,
    conn: Conn,
}

//pub struct client {
//    user: String,
//    pass: String,
//}

//let url = "mysql://root:password@localhost:3307/db_name";

//let pool = Pool::new(url)?;

//let mut conn = pool.get_conn()?;

impl Query {
    pub fn init(&mut self) {
        let opts = MyOpts {
            user: Some("root".to_string()),
            pass: Some("201314".to_string()),
            ..Default::default()
        };
        let p = MyPool::new(opts).unwrap();
        self.pool = p;
        let c = Conn::new(opts)?;
        self.conn = c;
    }

    pub fn closeConnection(&mut self) {
        if(self.conn) self.conn.close();
    }

    //pub fn queryFile(&mut self, path: String, name: String) -> FileItem {
        //let stmt = conn.prep("DO ?")?;
        //let sql = String::new();
        //write!(sql, "SELECT * FROM DFS.FILE WHERE NAME={} AND PATH='{}'",name,path);
        /*let rs = conn.prep_exec("SELECT path, name FROM DFS.FILE where path = :path and name = :name", 
            params!{"name" => name, "path" => path}).await?;
        let (_, user_info) = result.map_and_drop(|row| {
            /*let id: Option<i32> = row.get("id");
            let name: Option<String> = row.get("name");
            let path: Option<String> = row.get("path");
            let attribute: Option<String> = row.get("attribute");
            let time: Option<String> = row.get("time");
            let noa: Option<i32> = row.get("noa");
            let isFolder: Option<bool> = row.get("isFolder");
    
            (id, name, path, attribute, time, noa, isFolder)*/
            fileItem = FileItem {
                id: row.get("id"),
                path,
                name,
                attribute: row.get("attribute"),
                noa: row.get("noa"),
                time: row.get("time"),
                isFolder: row.get("isFolder"),
            }

        }).await?;*/

        /*let selected_file = self.conn
        .query_map(
            "SELECT path, name from FILE",
            |(path, name)| {
                FileItem { id, name, path... }  //?
            },
        )?;*/
        //let stmt = conn.prep("SELECT :foo, :bar, :foo")?;
        /*self.conn.exec_batch(
            r"SELECT FROM DFS.FILE WHERE FileItem (id, name, path, attribute, time, noa, is_folder)
              VALUES (?, :name, :path, ?, ?, ?, ?)",
            FileItem.iter().map(|p| params! {
                "name" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            })
        )?;*/
    //}

    pub fn addFile(file: FileItem) -> i32 {
        /*for mut stmt in pool.prepare(r"INSERT INTO tmp.payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (?, ?, ?)").into_iter() {
            for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happended.
                stmt.execute((p.customer_id, p.amount, &p.account_name)).unwrap();
            }
        }*/

        conn.exec_batch(
            r"INSERT INTO FILE (id, name, path, attribute, time, noa, isFolder)
              VALUES (:id, :name, :path, :attribute, :time, :noa, :isFolder)",
            payments.iter().map(|file| params! {
                "id" => file.id,
                "name" => &file.name,
                "path" => &file.path,
                "attribute" => &file.attribute,
                "time" => &file.time,
                "noa" => file.noa,
                "isFolder" => file.isFolder,
            })
        )?;
    }

    /*pub fn deleteFile(id: i32) -> i32 {
        
    }*/
}