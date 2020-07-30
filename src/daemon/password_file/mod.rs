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
use crate::passman_crypto;

use super::entry_value;

use self::regex::Regex;

type Result<T> = std::result::Result<T, String>;

pub struct PasswordFile {
    filename: String,
    key: String,
    is_open: bool,
    init_vec: [u8; 16],
    entries: HashMap<String, Vec<EntryValue>>,
}

fn vec_to_string(v: &mut Vec<EntryValue>) -> String {
    let v: Vec<String> = v.iter_mut().map(|x| x.to_string()).collect();
    v.join(";")
}

impl PasswordFile {
    pub fn new(filename: &str, key: &str) -> Result<Self> {
        let mut init_vec = [0; 16];
        OsRng.fill_bytes(&mut init_vec);

        let path = Path::new(&filename);
        if path.exists() { return Err("FileExistsAlready".to_string()); }

        println!("Path: {:?}", path);
        fs::write(path, format!("PASSMAN\n{}", init_vec.to_base64(STANDARD)).as_bytes()).map_err(|e| e.to_string())?;

        Ok(Self {
            filename: filename.to_string(),
            key: key.to_string(),
            is_open: false,
            init_vec,
            entries: HashMap::new(),
        })
    }

    ///
    ///
    pub fn open(filename: &str, key: &str) -> Result<PasswordFile> {

        // check if file exists
        let path = Path::new(filename);
        if !path.exists() { return Err("FileDoesNotExist".to_string()); }

        // read file content
        let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

        // extract initialization vector
        let init_vec = contents.split("\n").nth(1)
            .expect("Fill has no init vec.")
            .from_base64()
            .expect("Can not decode from base64");

        println!("Init Vec after: {:?}", <&[u8; 16]>::try_from(init_vec.as_slice()).unwrap());


        // extract and validate data
        let encrypted_data: Vec<String> = contents.split("\n").skip(2)
            .map(|x| x.to_string())
            .collect();
        let encrypted_data: String = encrypted_data.join("\n");

        println!("Encrypted data after: {:?}", encrypted_data);


        // TODO: decrypt data
        // let data = passman_crypto::decrypt(
        //     &encrypted_data.as_bytes().to_vec(),
        //     &key.to_string(),
        //     <&[u8; 16]>::try_from(init_vec.as_slice()).map_err(|err| err.to_string())?).unwrap();
        // let data = String::from_utf8(data.from_base64().map_err(|err| err.to_string())?)
        //     .map_err(|err| err.to_string())?;
        let data = encrypted_data;

        // validate data
        let regex = r"^((>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n)*(>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n*$";
        let re = Regex::new(regex).unwrap();
        if !data.is_empty() && !re.is_match(data.as_str()) {
            return Result::Err("Content is not proper formatted!".to_string());
        }

        // build the struct
        let mut psw_file = Self {
            filename: path.to_str().unwrap().to_owned(),
            key: key.to_string(),
            is_open: true,
            init_vec: <[u8; 16]>::try_from(init_vec.as_slice()).unwrap(),
            entries: HashMap::new(),
        };

        // parse data and load into struct
        PasswordFile::parse_file_content(&data, &mut psw_file.entries).unwrap();
        Result::Ok(psw_file)
    }

    pub fn close(password_file: &mut PasswordFile) -> Result<()> {

        // serialize data
        let data: String = password_file.entries.iter_mut()
            .map(|(key, val)| (key, vec_to_string(val.as_mut())))
            .map(|(key, val)| format!(">{}\n{}", key, val)).collect();

        // encrypt data
        // let encrypted_data = passman_crypto::encrypt(&data, &password_file.key, &password_file.init_vec).unwrap();
        // let encrypted_data = encrypted_data.to_base64(STANDARD);

        let encrypted_data = data;

        // create file and write content
        let path = Path::new(&password_file.filename);
        fs::write(path, format!("PASSMAN\n{}\n{}", password_file.init_vec.to_base64(STANDARD), encrypted_data).as_bytes()).unwrap();

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

    /// Deletes an entry from the password file.
    /// ```
    ///         let mut p = PasswordFile::new("fielname").unwrap();
    ///         p.add_entry("entry_name", vec![EntryValue::new("key","value")]);
    ///         assert!(p.get_entry("entry_name").is_ok());
    ///         p.delete_entry("entry_name");
    ///         assert!(p.get_entry("entry_name").is_err());
    /// ```
    pub fn delete_entry(&mut self, entry_name: &str) -> Result<()> {
        self.entries.remove(entry_name);
        Ok(())
    }

    /// Parses the data of a password file into a hashmap.
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice that holds the data from the file
    /// * `map` - Reference to the map, which the data is loaded into
    fn parse_file_content(data: &str, map: &mut HashMap<String, Vec<EntryValue>>) -> Result<()> {
        let entry_names: Vec<&str> = data.lines().filter(|x| x.starts_with(">")).collect();
        let lines: Vec<&str> = data.lines().collect();

        entry_names.into_iter().for_each(|name| {
            let idx = data.lines().position(|x| name == x).unwrap();
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
