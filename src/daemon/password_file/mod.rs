extern crate regex;

use std::fs;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use rand::RngCore;
use rand::rngs::OsRng;
use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};

use self::regex::Regex;

pub struct PasswordFile {
    filename: String,
    is_open: bool,
    init_vec: [u8; 16],
    entries: HashMap<String, Vec<(String, String)>>,
}

impl PasswordFile {
    pub fn new(filename: &str) -> Self {
        let mut init_vec = [0; 16];
        OsRng.fill_bytes(&mut init_vec);

        let path = Path::new(&filename);
        if !path.exists() {
            println!("Path: {:?}", path);
            let mut file = File::create(path).unwrap();
            file.write_all(format!("PASSMAN\n{}", init_vec.to_base64(STANDARD)).as_bytes()).unwrap();
        }

        Self {
            filename: filename.to_string(),
            is_open: false,
            init_vec,
            entries: HashMap::new(),
        }
    }

    pub fn open( password_file: &mut PasswordFile) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&password_file.filename)?;

        let cont: Vec<String> = contents.split("\n").skip(2)
            .map(|x| x.to_string())
            .collect();

        let vec = contents.split("\n").nth(1)
            .expect("Fill has no init vec.")
            .from_base64()
            .expect("Can not decode from base64");

        password_file.init_vec = <[u8; 16]>::try_from(vec.as_slice()).unwrap();

        let data: String = cont.join("\n");

        println!("Data: {}", data);
        let re = Regex::new(r"^((>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n)*(>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n*$").unwrap();
        if !data.is_empty() && !re.is_match(data.as_str()) {
            return Result::Err("Content is not proper formatted!")?;
        }

        PasswordFile::parse_file_content(&data, &mut password_file.entries).unwrap();
        password_file.is_open = true;
        Result::Ok(())
    }

    pub fn get_entry(&self, entry_name: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        match self.entries.get(entry_name) {
            Some(entry) => Ok(entry.to_owned()),
            None => Err(format!("No entry with name '{}'", entry_name))?,
        }
    }

    pub fn add_entry(&mut self, entry_name: &str, values: Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
        self.entries.insert(entry_name.to_string(), values);
        Ok(())
    }

    fn parse_file_content(content: &str, map: &mut HashMap<String, Vec<(String, String)>>) -> Result<(), Box<dyn Error>> {
        let entry_names: Vec<&str> = content.lines().filter(|x| x.starts_with(">")).collect();
        let lines: Vec<&str> = content.lines().collect();

        entry_names.into_iter().for_each(|name| {
            let idx = content.lines().position(|x| name == x).unwrap();
            let values: Vec<(String, String)> = lines
                .get(idx + 1)
                .unwrap()
                .split(";")
                .map(|key_value| {
                    let s: Vec<&str> = key_value.split(":").collect();
                    (s[0].to_string(), s[1].to_string())
                })
                .collect();
            map.insert(name.replace(">", ""), values);
        });
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests;
