use std::{error, fs};
use std::collections::HashMap;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

struct PasswordFile {
    filename: String,
    is_open: bool,
    entries: HashMap<String, Vec<(String, String)>>,
}

impl PasswordFile {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            is_open: false,
            entries: HashMap::new(),
        }
    }

    pub fn open(&self) -> Result<()> {
        let contents = match fs::read_to_string(&self.filename) {
            Ok(s) => s,
            Err(e) => format!("Something went wrong reading the file!\n{}", e),
        };

        // TODO: Implement
        return Result::Ok(());
    }

    fn parse_file_content(content: &String) -> HashMap<String, Vec<(String, String)>> {
        let mut map: HashMap<String, Vec<(String, String)>> = HashMap::new();

        let entry_names: Vec<&str> = content.lines().filter(|x| x.starts_with(">")).collect();
        let lines: Vec<&str> = content.lines().collect();

        entry_names.into_iter().for_each(|name| {
            let idx = content.lines().position(|x| name == x).unwrap();
            let values: Vec<(String, String)> = lines
                .get(idx + 1)
                .unwrap()
                .split(";")
                .map(|key_value| {
                    let s: Vec<String> = key_value.split(":").map(|a| a.to_string()).collect();
                    match &s[..] {
                        [first, second, ..] => (first.to_owned(), second.to_owned()),
                        _ => unreachable!(),
                    }
                })
                .collect();
            map.insert(name.replace(">", ""), values);
        });
        map
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Add;

    use crate::PasswordFile::PasswordFile;


    #[test]
    fn parse_file() {
        let file_content =
            String::from(">Gmail\nusername:julianriegraf@gmail.com;password:1234567890\n")
                .add(">Darknet\nusername:blackHat666;password:pwd\n")
                .add(">Internet\nusername:sexyBienchen68;password:strongPassword123\n");
        let map = PasswordFile::parse_file_content(&file_content);
        assert_eq!(map.len(), 3);

        assert!(map.get("Darknet").unwrap().contains(&(
            "username".to_string(),
            "blackHat666".to_string()
        )));

        assert!(map.get("Darknet").unwrap().contains(&(
            "password".to_string(),
            "pwd".to_string()
        )));

        assert!(map.get("Internet").unwrap().contains(&(
            "username".to_string(),
            "sexyBienchen68".to_string()
        )));

        assert!(map.get("Internet").unwrap().contains(&(
            "password".to_string(),
            "strongPassword123".to_string()
        )));
    }
}
