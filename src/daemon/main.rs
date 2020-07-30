use std::{env, fs, thread, time};
use std::borrow::Borrow;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{Shutdown, TcpListener};
use std::net::TcpStream;
use std::path::PathBuf;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use regex::Regex;

use crate::entry_value::EntryValue;
use crate::password_file::PasswordFile;

mod entry_value;
mod passman_crypto;
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

        buf_reader.read_line(&mut buffer).unwrap();

        println!("Message: '{}'", buffer);

        let response: Result<String> = match buffer
            .split(" ")
            .nth(0)
            .or(Option::Some("BAD REQUEST"))
            .unwrap()
        {
            "GET" => get(&password_file, &buffer),
            "ADD" => add(password_file.as_mut(), &buffer),
            "DELETE" => delete(password_file.as_mut(), &buffer),
            "CREATE" => {
                match create(&buffer) {
                    Ok(file) => {
                        password_file.replace(file);
                        assert!(password_file.is_some());
                        Ok("".to_string())
                    }
                    Err(e) => Err(e)
                }
            }
            "OPEN" => {
                match open(&buffer) {
                    Ok(pwd_file) => {
                        password_file = Some(pwd_file);
                        assert!(password_file.is_some());
                        Ok("".to_string())
                    }
                    Err(e) => Err(e)
                }
            }
            "CLOSE" => close(&mut password_file),
            _ => Err("BAD REQUEST".to_string()),
        };

        let response = response.map(|ok| format!("OK {}", ok)).map_err(|err| format!("ERR {}", err));
        stream.write_all(response.map_or_else(|s| s, |e| e).trim().as_bytes()).unwrap();
        stream.shutdown(Shutdown::Both).expect("Can not shutdown stream.");
    }
}

/// Refuses the connection to the server if the ip-address is not accepted
///
/// # Arguments
///
/// *`stream` - the used tcp stream
fn refuse_connection(stream: &mut TcpStream) {
    stream.write(format!("IP-Address ist not accepted!").as_bytes()).unwrap();
    stream.flush().unwrap();
}

/// Returns a Result<String> with either Ok or an Error.
///
/// # Arguments
///
/// *`password_file` - the password file, where data is added
/// *`message` - the request sent to the daemon
fn add(password_file: Option<&mut PasswordFile>, message: &String) -> Result<String> {
    let password_file = password_file.ok_or("NoOpenPasswordFile".to_string())?;
    if message.split(" ").count() < 2 { return Err("BAD REQUEST".to_string()); }
    let name = message.split(" ").nth(1).ok_or("BAD REQUEST".to_string())?;
    let key_values = match message.split(" ").nth(2) {
        Some(s) => s,
        None => return Err("BAD REQUEST ".to_string())
    };

    let re = Regex::new(r"^((([^;\n:]+:[^;\n:]+);)*([^;\n:]+:[^\n;:]+))\n*$").unwrap();
    if !key_values.is_empty() && !re.is_match(key_values) {
        return Result::Err("BAD REQUEST".to_string());
    }

    let vec: Vec<EntryValue> = key_values.split(";").map(|kv| {
        let a: Vec<&str> = kv.split(":").collect();
        EntryValue::new(a[0], a[1])
    }).collect();
    password_file.add_entry(&name, vec).or(Err("Adding the entry failed."))?;
    Ok("".to_string())
}

/// Returns a Result<String> with either Ok or an Error.
///
/// # Arguments
///
/// *`psw_file` - the password file, where data is deleted
/// *`message` - the request sent to the daemon
fn delete(psw_file: Option<&mut PasswordFile>, message: &String) -> Result<String> {
    let mut psw_file = psw_file.ok_or("NoOpenPasswordFile".to_string())?;
    psw_file.delete_entry(message.split(" ").nth(1).unwrap().borrow())
        .or(Err(format!("NotFound")))?;
    Ok("".to_string())
}

/// Returns a Result<String> with either the data for an account or an Error.
///
/// # Arguments
///
/// *`psw_file` - the password file, where data is deleted
/// *`message` - the request sent to the daemon
fn get(psw_file: &Option<PasswordFile>, message: &String) -> Result<String> {
    let psw_file = psw_file.as_ref().ok_or("NoOpenPasswordFile".to_string())?;
    let mut vec_result: Vec<EntryValue> = psw_file.get_entry(message.split(" ").nth(1).unwrap().borrow())
        .or(Err(format!("NotFound")))?;
    let v: Vec<String> = vec_result.iter_mut().map(|x| x.to_string()).collect();
    let msg = v.join(";");
    write_to_clipboard(msg.clone());
    Ok(msg)
}

/// Returns the path to the created password file or an error if the creation went wrong.
///
/// # Arguments
///
/// *`message` - the request sent to the daemon
fn create(message: &String) -> Result<PasswordFile> {
    let filename = message.split(" ").nth(1).ok_or("BAD_REQUEST".to_string())?;
    let key = message.split(" ").nth(2).ok_or("BAD_REQUEST".to_string())?;
    let path = env::var_os("HOME").unwrap();

    if fs::read_dir(&path).is_err() {
        fs::create_dir(&path).unwrap();
    }
    let path = PathBuf::from(path).join(".passman").join(&filename).as_path().with_extension("pass");
    password_file::PasswordFile::new(path.to_str().unwrap(), key)
}

/// Returns the path to the opened password file or an error if the open process went wrong.
///
/// # Arguments
///
/// *`message` - the request sent to the daemon
fn open(message: &String) -> Result<PasswordFile> {
    let filename = message.split(" ").nth(1).ok_or("BAD_REQUEST".to_string())?;
    let key = message.split(" ").nth(2).ok_or("BAD_REQUEST".to_string())?;
    let path = env::var_os("HOME").unwrap();
    let path = PathBuf::from(path).join(".passman").join(&filename).as_path().with_extension("pass");
    PasswordFile::open(&path.to_str().unwrap(), key)
}

/// Returns an empty string if the file is closed or an error if the close process failed.
///
/// # Arguments
///
/// *`psw_file_opt` - contains the password file to close
fn close(psw_file_opt: &mut Option<PasswordFile>) -> Result<String> {
    let psw_file = psw_file_opt.as_mut().ok_or("NoOpenPasswordFile".to_string())?;
    let a = PasswordFile::close(psw_file).map(|_| "".to_string()).map_err(|_| "Close failed".to_string());
    *psw_file_opt = None;
    a
}

fn write_to_clipboard(message: String) {
    let clp_thread = thread::spawn(move || {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(message.to_owned()).unwrap();
        let thirty_sec = time::Duration::from_secs(30);
        thread::sleep(thirty_sec);
        let content_after = ctx.get_contents().unwrap();
        if message == content_after {
            ctx.set_contents("".to_string()).unwrap();
        }
    });
}

#[cfg(test)]
mod tests {}
