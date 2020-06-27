extern crate reed_solomon_erasure;
#[macro_use]
extern crate lazy_static;

mod client;

fn main() {
    println!("Hello, world!");
    crate::client::client::client::main();
}


//mod server;