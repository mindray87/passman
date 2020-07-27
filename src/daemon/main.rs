use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Add;
use std::path::Path;

use rand::RngCore;
use rand::rngs::OsRng;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;

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
    stream.write(format!("IP-Address {} ist not accepted!", ip_address).as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {

    // TODO: Read until message ends
    // TODO: Encrypt socket connection
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        get()
    } else if buffer.starts_with(b"DELETE / HTTP/1.1\r\n") {
        delete()
    } else if buffer.starts_with(b"POST / HTTP/1.1\r\n") {
        post()
    } else {
        method_not_allowed()
    };

    let response = format!("{}{}", status_line, filename);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn method_not_allowed<'a>() -> (&'a str, &'a str) {
    ("HTTP/1.1 405 Method Not Allowed\r\n\r\n", "method_not_allowed")
}

fn get<'a>() -> (&'a str, &'a str) {
    // TODO: get the entry and return it
    ("HTTP/1.1 200 OK\r\n\r\n", "get")
}

fn post<'a>() -> (&'a str, &'a str) {
    // TODO: create a new entry
    ("HTTP/1.1 200 OK\r\n\r\n", "post")
}

fn delete<'a>() -> (&'a str, &'a str) {
    // TODO: delete an entry
    ("HTTP/1.1 200 OK\r\n\r\n", "delete")
}

fn open_password_file(filename: String) -> String {
    let contents = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => return format!("Something went wrong reading the file!\n{}", e)
    };

    return contents;
}

fn create_and_open_password_file(filename: &String) -> String {
    let filename = if filename.ends_with(".pass") { filename.to_string() } else { filename.to_string().add(".pass") };

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

// TODO: provide endpoints to open, close and create a new password database

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
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
        fs::remove_file(filename.add(".pass"));
        assert!(cont.starts_with("PASSMAN"));
    }
}