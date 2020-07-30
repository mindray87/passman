
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::ops::Add;

    const FILENAME: &str = "pw_file";

    #[test]
    fn parse_file() {
        let file_content =
            String::from(">Gmail\nusername:julianriegraf@gmail.com;password:1234567890\n")
                .add(">Darknet\nusername:blackHat666;password:pwd\n")
                .add(">Internet\nusername:sexyBienchen68;password:strongPassword123\n");

        let mut map: HashMap<String, Vec<EntryValue>> = HashMap::new();
        PasswordFile::parse_file_content(&file_content, &mut map).unwrap();
        assert_eq!(map.len(), 3);

        assert!(map
            .get("Darknet")
            .unwrap()
            .contains(&EntryValue::new("username", "blackHat666")));

        assert!(map.get("Darknet").unwrap().contains(&EntryValue::new("password", "pwd")));

        assert!(map
            .get("Internet")
            .unwrap()
            .contains(&EntryValue::new("username", "sexyBienchen68")));

        assert!(map
            .get("Internet")
            .unwrap()
            .contains(&EntryValue::new("password", "strongPassword123")));
    }

    #[test]
    fn open_file() {
        let mut paswd_file = PasswordFile::new(FILENAME, "1234567890123456").unwrap();
        let init_vec = paswd_file.init_vec;
        let values = vec![EntryValue::new("username", "u"), EntryValue::new("password", "1234")];
        paswd_file.entries.insert("Gmail".to_string(), values.clone());
        PasswordFile::close(&mut paswd_file).unwrap();
        let paswd_file = PasswordFile::open(FILENAME, "1234567890123456").unwrap();
        assert_eq!(paswd_file.get_entry("Gmail").unwrap(), values);
        assert_eq!(paswd_file.init_vec, init_vec);
        assert_eq!(paswd_file.key, "1234567890123456");
        assert!(paswd_file.is_open);
        fs::remove_file(FILENAME).unwrap();
    }

    #[test]
    fn create_file() {
        let paswd_file = PasswordFile::new(FILENAME, "key").unwrap();
        assert_eq!(paswd_file.entries.len(), 0);
        assert_eq!(paswd_file.is_open, false);
        fs::remove_file(FILENAME).unwrap();
    }

    #[test]
    fn get_entry() {
        let mut paswd_file = PasswordFile::new(FILENAME, "key").unwrap();
        let values = vec![EntryValue::new("username", "u"), EntryValue::new("password", "1234")];
        paswd_file.entries.insert("Gmail".to_string(), values.clone());
        assert_eq!(paswd_file.get_entry("Gmail").unwrap(), values);
        fs::remove_file(FILENAME).unwrap();
    }

    #[test]
    fn add_entry() {
        let mut paswd_file = PasswordFile::new(FILENAME, "key").unwrap();
        let username = EntryValue::new("username", "rustic expert");
        let password = EntryValue::new("password", "abc");
        let vec = vec![username, password];

        paswd_file.add_entry("Rust Nerds", vec.clone()).unwrap();
        assert_eq!(paswd_file.get_entry("Rust Nerds").unwrap(), vec);
        fs::remove_file(FILENAME).unwrap();
    }

    #[test]
    fn delete_entry() {
        let mut paswd_file = PasswordFile::new(FILENAME, "key").unwrap();
        let username = EntryValue::new("username", "rustic expert");
        let password = EntryValue::new("password", "abc");
        let vec = vec![username, password];

        paswd_file.add_entry("Rust Nerds", vec.clone()).unwrap();
        assert_eq!(paswd_file.get_entry("Rust Nerds").unwrap(), vec);

        paswd_file.delete_entry("Rust Nerds").unwrap();
        assert_eq!(paswd_file.get_entry("Rust Nerds").is_err(), true);
        fs::remove_file(FILENAME).unwrap();
    }

    #[test]
    fn de_and_encrypt() {
        let key = "aaaaaaaaaaaaaaaa";
        fs::remove_file(FILENAME).unwrap_or(());
        let mut paswd_file = PasswordFile::new(FILENAME, key).unwrap();
        let vec = vec![EntryValue::new("username", "rustic_expert"), EntryValue::new("password", "abc")];

        paswd_file.add_entry("Rust_Nerds", vec.clone()).unwrap();
        assert_eq!(paswd_file.get_entry("Rust_Nerds").unwrap(), vec);
        PasswordFile::close(&mut paswd_file).unwrap();

        let paswd_file = PasswordFile::open(FILENAME, key).unwrap();
        assert_eq!(paswd_file.get_entry("Rust_Nerds").unwrap(), vec);
        fs::remove_file(FILENAME).unwrap();
    }
}