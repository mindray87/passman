extern crate regex;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

use rand::RngCore;
use rand::rngs::OsRng;
use rustc_serialize::base64::{FromBase64, STANDARD, ToBase64};

use crate::entry_value::EntryValue;
use crate::passman_crypto;

use self::regex::Regex;

type Result<T> = std::result::Result<T, String>;


/// Representation of a password file
pub struct PasswordFile {
    filename: String,
    key: String,
    is_open: bool,
    init_vec: [u8; 16],
    entries: HashMap<String, Vec<EntryValue>>,
}

/// Returns the String value of a Vec
///
/// # Arguments
///
/// *`v` - the vector of Entry values
fn vec_to_string(v: &mut Vec<EntryValue>) -> String {
    let v: Vec<String> = v.iter_mut().map(|x| x.to_string()).collect();
    v.join(";")
}

/// Returns a Result String for the key validation process
///
/// # Arguments
///
/// *`key` - the encryption key
fn validate_key(key: &str) -> Result<String> {
    if key.len() > 16 { return Err("KeyLimitExceeded".to_string()); }
    let k = &mut [65 as u8; 16];
    key.as_bytes().into_iter().enumerate().for_each(|(i, x)| k[i] = *x);
    Ok(String::from_utf8(k.to_vec()).map_err(|err| err.to_string())?)
}

impl PasswordFile {
    /// Returns a newly created Passwordfile
    ///
    /// # Arguments
    ///
    /// *`filename` - the name of the file
    /// *`key` - the encryption key for the file
    pub fn new(filename: &str, key: &str) -> Result<Self> {
        let key = validate_key(key)?;
        let mut init_vec = [0; 16];
        OsRng.fill_bytes(&mut init_vec);

        let path = Path::new(&filename);
        if path.exists() { return Err("FileExistsAlready".to_string()); }

        fs::write(path, format!("PASSMAN\n{}", init_vec.to_base64(STANDARD)).as_bytes()).map_err(|e| e.to_string())?;

        Ok(Self {
            filename: filename.to_string(),
            key,
            is_open: false,
            init_vec,
            entries: HashMap::new(),
        })
    }

    /// Returns the handle for a Passwordfile
    ///
    /// # Arguments
    ///
    /// *`filename` - the name of the password file
    /// *`key` - the encryption key for the file
    pub fn open(filename: &str, key: &str) -> Result<PasswordFile> {

        // validate the key
        let key = validate_key(key)?;

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

        // extract and validate data
        let encrypted_data: Vec<String> = contents.split("\n").skip(2)
            .map(|x| x.to_string())
            .collect();
        let encrypted_data: String = encrypted_data.join("\n");

        //  decrypt data
        let encrypted_data = encrypted_data.from_base64().map_err(|err| err.to_string())?;
        let data = passman_crypto::decrypt(
            &encrypted_data,
            &key.to_string(),
            <&[u8; 16]>::try_from(init_vec.as_slice()).map_err(|err| err.to_string())?)?;

        // validate data
        let regex = r"^((>.+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n)*(>.+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n*$";
        let re = Regex::new(regex).unwrap();
        if !data.is_empty() && !re.is_match(data.as_str()) {
            return Result::Err("Content is not proper formatted!".to_string());
        }

        // build the struct
        let mut psw_file = Self {
            filename: path.to_str().unwrap().to_owned(),
            key,
            is_open: true,
            init_vec: <[u8; 16]>::try_from(init_vec.as_slice()).unwrap(),
            entries: HashMap::new(),
        };

        // parse data and load into struct
        PasswordFile::parse_file_content(&data, &mut psw_file.entries).unwrap();
        Result::Ok(psw_file)
    }

    /// Returns the result of the closing process of an openend password file
    ///
    /// # Arguments
    /// *`password_file` - the opened password file
    pub fn close(password_file: &mut PasswordFile) -> Result<()> {

        // serialize data
        let data: String = password_file.entries.iter_mut()
            .map(|(key, val)| (key, vec_to_string(val.as_mut())))
            .map(|(key, val)| format!(">{}\n{}\n", key, val)).collect();

        // encrypt data
        let encrypted_data = passman_crypto::encrypt(&data, &password_file.key, &password_file.init_vec)?;
        let encrypted_data = encrypted_data.to_base64(STANDARD);

        // create file and write content
        let path = Path::new(&password_file.filename);
        fs::write(path, format!("PASSMAN\n{}\n{}", password_file.init_vec.to_base64(STANDARD), encrypted_data).as_bytes()).unwrap();

        password_file.is_open = false;
        Result::Ok(())
    }

    /// Returns the Entry for an account in a password file
    ///
    /// # Arguments
    ///
    /// *`entry_name` - the accountname stored in the password file
    pub fn get_entry(&self, entry_name: &str) -> Result<Vec<EntryValue>> {
        match self.entries.get(entry_name) {
            Some(entry) => Ok(entry.to_vec()),
            None => Err(format!("No entry with name '{}'", entry_name))?,
        }
    }

    /// Returns the result of the add an entry to the password file process.
    ///
    /// # Arguments
    ///
    /// *`entry_name` - the accountname for a password file
    /// *`values` - the values, e.g. username and password
    pub fn add_entry(&mut self, entry_name: &str, values: Vec<EntryValue>) -> Result<()> {
        self.entries.insert(entry_name.to_string(), values);
        Ok(())
    }

    /// Returns the result of Deleting an entry from the password file.
    ///
    /// # Arguments
    ///
    /// *`entry_name` - Accountname of the entry
    ///
    /// # Examples
    ///
    /// ```
    ///         let mut p = PasswordFile::new("filename").unwrap();
    ///         p.add_entry("entry_name", vec![EntryValue::new("key","value")]);
    ///         assert!(p.get_entry("entry_name").is_ok());
    ///         p.delete_entry("entry_name");
    ///         assert!(p.get_entry("entry_name").is_err());
    /// ```
    pub fn delete_entry(&mut self, entry_name: &str) -> Result<()> {
        match self.entries.remove(entry_name) {
            Some(_) => Ok(()),
            None => Err("NotFound".to_string())
        }
    }

    /// Returns the result of parsing the data of a password file into a hashmap.
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