//pub mod FileAttrs;
//pub mod DeviceItem;
//pub mod FileItem;
//pub mod FileUploader;
//pub mod RequestItem;
//pub mod Query;
#[macro_use]
extern crate mysql;
// ...

use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct FileItem {
    id: i32,
    name: Option<String>,
    path: Option<String>,
    attribute: Option<String>,
    time: Option<String>,
    noa: i32,
    is_folder: bool,
}


fn main() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let pool = my::Pool::new("mysql://root:mysql@localhost:3306/mysql").unwrap();

    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(r"CREATE TABLE FILE(
                    id int NOT NULL AUTO_INCREMENT,   
                    name char(20) NOT NULL DEFAULT '',   
                    path char(60) NOT NULL DEFAULT '',   
                    attribute char(10) NOT NULL DEFAULT '',   
                    time char(10) NOT NULL DEFAULT '',   
                    noa int NOT NULL DEFAULT 1,   
                    is_folder boolean NOT NULL DEFAULT false,   
                    PRIMARY KEY (`id`) 
                    )", ()).unwrap();

    let files = vec![
        FileItem { id: 1, name: Some("a".into()), path: Some("root".into()), 
            attribute: Some("rw".into()), time: Some("c".into()), noa: 1, is_folder: false },
        FileItem { id: 2, name: Some("b".into()), path: Some("root".into()), 
            attribute: Some("ro".into()), time: Some("c".into()), noa: 2, is_folder: false },
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO FILE
                                       (id, name, path, attribute, time, noa, is_folder)
                                   VALUES
                                       (:id, :name, :path, :attribute, :time, :noa, :is_folder)").into_iter() {
        for p in files.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "id" => p.id,
                "name" => &p.name,
                "path" => &p.path,
                "attribute" => &p.attribute,
                "time" => &p.time,
                "noa" => p.noa,
                "is_folder" => p.is_folder,
            }).unwrap();
        }
    }

    // Let's select payments from database
    /*let selected_payments: Vec<Payment> =
    pool.prep_exec("SELECT customer_id, amount, account_name from payment", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            //  Note that from_row will panic if you don't follow your schema
            let (customer_id, amount, account_name) = my::from_row(row);
            Payment {
                customer_id: customer_id,
                amount: amount,
                account_name: account_name,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lukky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");*/
}

