extern crate regex;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::RngCore;
use rand::rngs::OsRng;
use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};

use crate::entry_value::EntryValue;

use super::entry_value;

use self::regex::Regex;

type Result<T> = std::result::Result<T, String>;

pub struct PasswordFile {
    filename: String,
    is_open: bool,
    init_vec: [u8; 16],
    entries: HashMap<String, Vec<EntryValue>>,
}

fn vec_to_string(v: &mut Vec<EntryValue>) -> String {
    let v: Vec<String> = v.iter_mut().map(|x| x.to_str()).collect();
    v.join(";")
}

impl PasswordFile {
    pub fn new(filename: &str) -> Result<Self> {
        let mut init_vec = [0; 16];
        OsRng.fill_bytes(&mut init_vec);

        let path = Path::new(&filename);
        if path.exists() { return Err("FileExistsAlready".to_string()); }

        println!("Path: {:?}", path);
        fs::write(path, format!("PASSMAN\n{}", init_vec.to_base64(STANDARD)).as_bytes()).map_err(|e| e.to_string())?;

        Ok(Self {
            filename: filename.to_string(),
            is_open: false,
            init_vec,
            entries: HashMap::new(),
        })
    }

    pub fn open(filename: &str) -> Result<PasswordFile> {

        let path = Path::new(filename);
        if !path.exists() { return Err("FileDoesNotExist".to_string()); }

        let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

        let cont: Vec<String> = contents.split("\n").skip(2)
            .map(|x| x.to_string())
            .collect();

        let vec = contents.split("\n").nth(1)
            .expect("Fill has no init vec.")
            .from_base64()
            .expect("Can not decode from base64");

        let mut psw_file = Self {
            filename: path.to_str().unwrap().to_owned(),
            is_open: true,
            init_vec: <[u8; 16]>::try_from(vec.as_slice()).unwrap(),
            entries: HashMap::new(),
        };

        let data: String = cont.join("\n");

        let re = Regex::new(r"^((>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n)*(>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n*$").unwrap();
        if !data.is_empty() && !re.is_match(data.as_str()) {
            return Result::Err("Content is not proper formatted!".to_string());
        }

        PasswordFile::parse_file_content(&data, &mut psw_file.entries).unwrap();
        Result::Ok(psw_file)
    }

    pub fn close(password_file: &mut PasswordFile) -> Result<()> {
        let data: String = password_file.entries.iter_mut()
            .map(|(key, val)| (key, vec_to_string(val.as_mut())))
            .map(|(key, val)| format!(">{}\n{}", key, val)).collect();

        let path = Path::new(&password_file.filename);
        fs::write(path, format!("PASSMAN\n{}\n{}", password_file.init_vec.to_base64(STANDARD), data).as_bytes()).unwrap();

        password_file.is_open = false;
        Result::Ok(())
    }

    pub fn get_entry(&self, entry_name: &str) -> Result<Vec<EntryValue>> {
        match self.entries.get(entry_name) {
            Some(entry) => Ok(entry.to_vec()),
            None => Err(format!("No entry with name '{}'", entry_name))?,
        }
    }

    pub fn add_entry(&mut self, entry_name: &str, values: Vec<EntryValue>) -> Result<()> {
        self.entries.insert(entry_name.to_string(), values);
        Ok(())
    }

    pub fn delete_entry(&mut self, entry_name: &str) -> Result<()> {
        self.entries.remove(entry_name);
        Ok(())
    }

    fn parse_file_content(content: &str, map: &mut HashMap<String, Vec<EntryValue>>) -> Result<()> {
        let entry_names: Vec<&str> = content.lines().filter(|x| x.starts_with(">")).collect();
        let lines: Vec<&str> = content.lines().collect();

        entry_names.into_iter().for_each(|name| {
            let idx = content.lines().position(|x| name == x).unwrap();
            let values: Vec<EntryValue> = lines
                .get(idx + 1)
                .unwrap()
                .split(";")
                .map(|key_value| {
                    let s: Vec<&str> = key_value.split(":").collect();
                    EntryValue::new(s[0], s[1])
                })
                .collect();
            map.insert(name.replace(">", ""), values);
        });
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests;
