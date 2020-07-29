use std::collections::HashMap;
use std::fs;
use std::ops::Add;

use crate::password_file::PasswordFile;

#[test]
fn parse_file() {
    let file_content =
        String::from(">Gmail\nusername:julianriegraf@gmail.com;password:1234567890\n")
            .add(">Darknet\nusername:blackHat666;password:pwd\n")
            .add(">Internet\nusername:sexyBienchen68;password:strongPassword123\n");

    let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();
    PasswordFile::parse_file_content(&file_content, &mut map).unwrap();
    assert_eq!(map.len(), 3);

    assert!(map
        .get("Darknet")
        .unwrap()
        .contains(&("username".to_string(), "blackHat666".to_string())));

    assert!(map.get("Darknet").unwrap().contains(&("password".to_string(), "pwd".to_string())));

    assert!(map
        .get("Internet")
        .unwrap()
        .contains(&("username".to_string(), "sexyBienchen68".to_string())));

    assert!(map
        .get("Internet")
        .unwrap()
        .contains(&("password".to_string(), "strongPassword123".to_string())));
}

#[test]
fn open_file() {
    let mut paswd_file = PasswordFile::new("src/daemon/password_file/test_password_files/password_file.pass");
    match PasswordFile::open(&mut paswd_file) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    assert_eq!(paswd_file.entries.len(), 3);
    assert_eq!(paswd_file.init_vec, [233, 41, 226, 105, 74, 238, 246, 25, 38, 245, 142, 173, 133, 134, 159, 142]);
    assert!(paswd_file.is_open);
}

#[test]
fn create_file() {
    let paswd_file = PasswordFile::new("src/daemon/password_file/test_password_files/password_file01.pass");
    assert_eq!(paswd_file.entries.len(), 0);
    println!("init vec: {:?}", paswd_file.init_vec);
    assert_eq!(paswd_file.is_open, false);
    fs::remove_file(paswd_file.filename).unwrap();
}

#[test]
fn get_entry() {
    let mut paswd_file = PasswordFile::new("src/daemon/password_file/test_password_files/password_file.pass");
    match PasswordFile::open(&mut paswd_file) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
    assert_eq!(paswd_file.get_entry("Gmail").unwrap(),
               vec![("username".to_string(),
                     "julianriegraf@gmail.com".to_string()),
                    ("password".to_string()
                     , "1234567890".to_string()
                    )]);
}

#[test]
fn add_entry() {
    let mut paswd_file = PasswordFile::new("src/daemon/password_file/test_password_files/password_file.pass");
    match PasswordFile::open(&mut paswd_file) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }

    let username = (String::from("username"), String::from("rustic expert"));
    let password = (String::from("password"), String::from("abc"));
    let vec = vec![username, password];

    paswd_file.add_entry("Rust Nerds", vec.clone()).unwrap();
    assert_eq!(paswd_file.get_entry("Rust Nerds").unwrap(), vec);
}
