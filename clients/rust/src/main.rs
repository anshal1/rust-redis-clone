mod client;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::client::MyHashClient;
fn main() {
    let file = File::open(Path::new("./names.txt")).unwrap();
    let buf = BufReader::new(file);

    let mut connection = match MyHashClient::Client::new("127.0.0.1", 5000) {
        Ok(conn) => {
            println!("Connection successfull");
            conn
        }
        Err(e) => {
            println!("Unable to connect to the remove server, {}", e);
            return;
        }
    };
    let mut idx = 0;
    for name in buf.lines() {
        let name = name.unwrap();
        if let Some(err) = connection.add(&idx.to_string(), &name) {
            println!("Error: {}", err);
            return;
        }
        idx += 1;
    }
    for i in 0..idx {
        if let Some(err) = connection.del(&i.to_string()) {
            println!("Error: {}", err);
            continue;
        }
    }

    connection.disconnect().unwrap();

    println!("Hello, world!");
}
