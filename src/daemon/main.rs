use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Add;
use std::path::Path;

use crypto::buffer::RefWriteBuffer;
use rand::rngs::OsRng;
use rand::RngCore;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;

mod password_file;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip_address = stream.local_addr().expect("Could not read address").ip();
        if !ip_address.is_loopback() {
            refuse_connection(stream, ip_address.to_string());
            continue;
        }
        handle_connection(stream);
    }
}

fn refuse_connection(mut stream: TcpStream, ip_address: String) {
    stream
        .write(format!("IP-Address {} ist not accepted!", ip_address).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer);

    let response = match buffer
        .split(" ")
        .nth(0)
        .or(Option::Some("BAD REQUEST"))
        .unwrap()
    {
        "GET" => get(&buffer),
        "ADD" => add(&buffer),
        "DELETE" => delete(&buffer),
        "CREATE" => create(&buffer),
        "OPEN" => open(&buffer),
        "CLOSE" => close(&buffer),
        _ => "BAD REQUEST",
    };

    println!("Response: '{}'", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn add(message: &String) -> &'static str {
    message
        .split("\n")
        .filter(|x| !x.is_empty())
        .for_each(|line| println!("'{}'", line));
    "OK"
}

fn delete(message: &String) -> &'static str {
    "NOT IMPLEMENTED"
}

fn get(message: &String) -> &'static str {
    "NOT IMPLEMENTED"
}

fn create(message: &String) -> &'static str {
    "NOT IMPLEMENTED"
}

fn open(message: &String) -> &'static str {
    "NOT IMPLEMENTED"
}

fn close(message: &String) -> &'static str {
    "NOT IMPLEMENTED"
}

fn open_password_file(filename: String) -> String {
    let contents = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => return format!("Something went wrong reading the file!\n{}", e),
    };

    return contents;
}

fn create_and_open_password_file(filename: &String) -> String {
    let filename = if filename.ends_with(".pass") {
        filename.to_string()
    } else {
        filename.to_string().add(".pass")
    };

    let path = Path::new(filename.as_str());
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut init_vec: [u8; 16] = [0; 16];
    OsRng.fill_bytes(&mut init_vec);

    match file.write_all(format!("PASSMAN\n{}", init_vec.to_base64(base64::STANDARD)).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    return open_password_file(filename);
}

fn close_password_file() {}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::ops::Add;

    use crate::{create_and_open_password_file, open_password_file};

    #[test]
    fn open_password_file_fails() {
        let filename = String::from("this file does not exist");
        assert!(open_password_file(filename).starts_with("Something went wrong reading the file"));
    }

    #[test]
    fn create_password_file() {
        let filename = String::from("my_test_password_file");
        let cont = create_and_open_password_file(&filename);
        println!("content: {}", cont);
        fs::remove_file(filename.add(".pass")).unwrap();
        assert!(cont.starts_with("PASSMAN"));
    }
}
