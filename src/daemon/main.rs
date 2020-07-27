use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::stdin;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
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

// TODO: provide endpoints to open, close and create a new password database