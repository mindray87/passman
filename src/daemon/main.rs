use std::{env, fs};
use std::borrow::{Borrow, BorrowMut};
use std::convert::TryFrom;
use std::error::Error;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ops::Add;
use std::path::{Path, PathBuf};

use password_file::PasswordFile;

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
        let mut stream = stream.unwrap();
        let ip_address = stream.local_addr().expect("Could not read address").ip();
        if !ip_address.is_loopback() {
            refuse_connection(stream, ip_address.to_string());
            continue;
        }

        // handle_connection(stream);
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();

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
        stream.write(format!("{}", response.map_or_else(|s| s, |e| e)).as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn refuse_connection(mut stream: TcpStream, ip_address: String) {
    stream
        .write(format!("IP-Address {} ist not accepted!", ip_address).as_bytes())
        .unwrap();
    stream.flush().unwrap();
}

fn add(password_file: Option<&mut PasswordFile>, message: &String) -> Result<String> {
    let mut password_file = password_file.ok_or("There is no password file open.".to_string())?;
    let name = message.lines().nth(0).unwrap().replace("ADD ", "");
    let key_values = match message.split("\n").nth(1) {
        Some(s) => s,
        None => return Err("BAD RESPIONSE ".to_string())
    };
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
    let mut filename = message.lines().nth(0).unwrap().replace("CREATE ", "");
    let path = env::var_os("HOME")
        .map(PathBuf::from)
        .map(|x| x.join(&filename))
        .unwrap();

    let path = path.as_path().with_extension(".pass");

    match path.to_str() {
        Some(s) => Ok(password_file::PasswordFile::new(s)),
        None => Err("There is something wrong with the path!".to_string())
    }
}

fn open(message: &String) -> Result<PasswordFile> {
    let mut filename = message.lines().nth(0).unwrap().replace("OPEN ", "");
    let path = env::var_os("HOME")
        .map(PathBuf::from)
        .map(|x| x.join(&filename))
        .unwrap();

    let path = path.as_path().with_extension(".pass");

    let mut password_file = match path.to_str() {
        Some(s) => password_file::PasswordFile::new(s),
        None => return Err("There is something wrong with the path!".to_string())
    };
    PasswordFile::open(&mut password_file).map(|e| password_file).map_err(|e| "Open failed".to_string())
}

fn close(message: &String) -> Result<String> {
    Err("NOT IMPLEMENTED")?
}

fn open_password_file(filename: String) -> String {
    let contents = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => return format!("Something went wrong reading the file!\n{}", e),
    };

    return contents;
}


fn close_password_file() {}

#[cfg(test)]
mod tests {
    use crate::open_password_file;

    #[test]
    fn open_password_file_fails() {
        let filename = String::from("this file does not exist");
        assert!(open_password_file(filename).starts_with("Something went wrong reading the file"));
    }

    // #[test]
    // fn create_password_file() {
    //     let filename = String::from("my_test_password_file");
    //     let cont = create_and_open_password_file(&filename);
    //     println!("content: {}", cont);
    //     fs::remove_file(filename.add(".pass")).unwrap();
    //     assert!(cont.starts_with("PASSMAN"));
    // }
}
