use std::collections::HashMap;
use std::fs;
use std::ops::Add;

use crate::key_value::KeyValue;
use crate::password_file::PasswordFile;

#[test]
fn parse_file() {
    let file_content =
        String::from(">Gmail\nusername:julianriegraf@gmail.com;password:1234567890\n")
            .add(">Darknet\nusername:blackHat666;password:pwd\n")
            .add(">Internet\nusername:sexyBienchen68;password:strongPassword123\n");

    let mut map: HashMap<String, Vec<KeyValue>> = HashMap::new();
    PasswordFile::parse_file_content(&file_content, &mut map).unwrap();
    assert_eq!(map.len(), 3);

    assert!(map
        .get("Darknet")
        .unwrap()
        .contains(&KeyValue::new("username", "blackHat666")));

    assert!(map.get("Darknet").unwrap().contains(&KeyValue::new("password", "pwd")));

    assert!(map
        .get("Internet")
        .unwrap()
        .contains(&KeyValue::new("username", "sexyBienchen68")));

    assert!(map
        .get("Internet")
        .unwrap()
        .contains(&KeyValue::new("password", "strongPassword123")));
}

#[test]
fn open_file() {
    let mut paswd_file = PasswordFile::open("src/daemon/password_file/test_password_files/password_file.pass").unwrap();
    assert_eq!(paswd_file.entries.len(), 3);
    assert_eq!(paswd_file.init_vec, [233, 41, 226, 105, 74, 238, 246, 25, 38, 245, 142, 173, 133, 134, 159, 142]);
    assert!(paswd_file.is_open);
}

#[test]
fn create_file() {
    let paswd_file = PasswordFile::new("src/daemon/password_file/test_password_files/password_file01.pass").unwrap();
    assert_eq!(paswd_file.entries.len(), 0);
    println!("init vec: {:?}", paswd_file.init_vec);
    assert_eq!(paswd_file.is_open, false);
    fs::remove_file(paswd_file.filename).unwrap();
}

#[test]
fn get_entry() {
    let mut paswd_file = PasswordFile::open("src/daemon/password_file/test_password_files/password_file.pass").unwrap();
    assert_eq!(paswd_file.get_entry("Gmail").unwrap(),
               vec![KeyValue::new("username", "julianriegraf@gmail.com"), KeyValue::new("password", "1234567890")]);
}

#[test]
fn add_entry() {
    let mut paswd_file = PasswordFile::open("src/daemon/password_file/test_password_files/password_file.pass").unwrap();
    let username = KeyValue::new("username", "rustic expert");
    let password = KeyValue::new("password", "abc");
    let vec = vec![username, password];

    paswd_file.add_entry("Rust Nerds", vec.clone()).unwrap();
    assert_eq!(paswd_file.get_entry("Rust Nerds").unwrap(), vec);
}

#[test]
fn delete_entry() {
    let mut paswd_file = PasswordFile::open("src/daemon/password_file/test_password_files/password_file.pass").unwrap();
    let username = KeyValue::new("username", "rustic expert");
    let password = KeyValue::new("password", "abc");
    let vec = vec![username, password];

    paswd_file.add_entry("Rust Nerds", vec.clone()).unwrap();
    assert_eq!(paswd_file.get_entry("Rust Nerds").unwrap(), vec);

    paswd_file.delete_entry("Rust Nerds").unwrap();
    assert_eq!(paswd_file.get_entry("Rust Nerds").is_err(), true);

}