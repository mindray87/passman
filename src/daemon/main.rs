extern crate clipboard;
use std::{env, fs, time};
use std::borrow::Borrow;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, Shutdown};
use std::net::TcpStream;
use std::path::PathBuf;

use regex::Regex;

use password_file::PasswordFile;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
mod password_file;

type Result<T> = std::result::Result<T, String>;

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:7878") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let mut password_file: Option<PasswordFile> = None;

    for stream in listener.incoming() {
        let mut stream = stream.expect("Stream error!");

        if !stream.local_addr().unwrap().ip().is_loopback() { refuse_connection(&mut stream) }

        let mut buf_reader = BufReader::new(&stream);
        let mut buffer = String::new();

        buf_reader.read_to_string(&mut buffer).unwrap();

        println!("Message: '{}'", buffer);

        let response: Result<String> = match buffer
            .split(" ")
            .nth(0)
            .or(Option::Some("BAD REQUEST"))
            .unwrap()
        {
            "GET" => get(&password_file, &buffer),
            "ADD" => add(password_file.as_mut(), &buffer),
            "DELETE" => delete(&buffer),
            "CREATE" => {
                match create(&buffer) {
                    Ok(file) => {
                        password_file.replace(file);
                        assert!(password_file.is_some());
                        Ok("OK".to_string())
                    }
                    Err(e) => Err(e)
                }
            }
            "OPEN" => {
                match open(&buffer) {
                    Ok(pwd_file) => {
                        password_file = Some(pwd_file);
                        assert!(password_file.is_some());
                        Ok("OK".to_string())
                    }
                    Err(e) => Err(e)
                }
            }
            "CLOSE" => close(&buffer),
            _ => Err("BAD REQUEST".to_string()),
        };

        println!("Response: '{:#?}'", response);
        stream.write_all(format!("{}", response.map_or_else(|s| s, |e| e)).as_bytes()).unwrap();
        stream.shutdown(Shutdown::Both).expect("Can not shutdown stream.");
    }
}

fn refuse_connection(stream: &mut TcpStream) {
    stream.write(format!("IP-Address ist not accepted!").as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn add(password_file: Option<&mut PasswordFile>, message: &String) -> Result<String> {
    let password_file = password_file.ok_or("There is no password file open.".to_string())?;
    if message.lines().count() < 2 { return Err("BAD REQUEST".to_string()); }
    let name = message.lines().nth(0).unwrap().replace("ADD ", "");
    let key_values = match message.split("\n").nth(1) {
        Some(s) => s,
        None => return Err("BAD REQUEST ".to_string())
    };

    let re = Regex::new(r"^((([^;\n:]+:[^;\n:]+);)*([^;\n:]+:[^\n;:]+))\n*$").unwrap();
    if !key_values.is_empty() && !re.is_match(key_values) {
        return Result::Err("Content is not proper formatted!".to_string());
    }

    let vec: Vec<(String, String)> = key_values.split(";").map(|kv| {
        let a: Vec<&str> = kv.split(":").collect();
        (a[0].to_string(), a[1].to_string())
    }).collect();
    password_file.add_entry(&name, vec).or(Err("Adding the entry failed."))?;
    Ok("OK".to_string())
}

fn delete(message: &String) -> Result<String> {
    Err("NOT IMPLEMENTED".to_string())
}

fn get(password_file: &Option<PasswordFile>, message: &String) -> Result<String> {
    let password_file = password_file.as_ref().ok_or("There is no password file open.".to_string())?;
    let vec_result: Vec<(String, String)> = password_file.get_entry(message.lines().nth(0).unwrap().replace("GET ", "").borrow())
        .or(Err(format!("ERR\nEntry not found.")))?;

    Ok(format!("OK\n{:?}", vec_result))
}


fn create(message: &String) -> Result<PasswordFile> {
    let filename = message.lines().nth(0).unwrap().replace("CREATE ", "");
    let path = env::var_os("HOME").unwrap();

    if fs::read_dir(&path).is_err() {
        fs::create_dir(&path).unwrap();
    }
    let path = PathBuf::from(path).join(".passman").join(&filename).as_path().with_extension("pass");

    match path.to_str() {
        Some(s) => Ok(password_file::PasswordFile::new(s)),
        None => Err("There is something wrong with the path!".to_string())
    }
}

fn open(message: &String) -> Result<PasswordFile> {
    let filename = message.lines().nth(0).unwrap().replace("OPEN ", "");
    let path = env::var_os("HOME")
        .map(PathBuf::from)
        .map(|x| x.join(&filename))
        .unwrap();

    let path = path.as_path().with_extension(".pass");

    let mut password_file = match path.to_str() {
        Some(s) => password_file::PasswordFile::new(s),
        None => return Err("There is something wrong with the path!".to_string())
    };
    PasswordFile::open(&mut password_file).map(|_| password_file).map_err(|_| "Open failed".to_string())
}

fn close(message: &String) -> Result<String> {
    Err("NOT IMPLEMENTED")?
}

fn close_password_file() {}

#[cfg(test)]
mod tests {}
