extern crate regex;

use std::collections::HashMap;
use std::error::Error;
use std::{error, fs};

use self::regex::Regex;

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

    pub fn open(&self) -> Result<(),Box<dyn Error>> {
        let contents = match fs::read_to_string(&self.filename) {
            Ok(s) => s,
            Err(e) => format!("Something went wrong reading the file!\n{}", e),
        };

        // TODO: Implement
        return Result::Ok(());
    }

    fn parse_file_content(content: &str) -> Result<HashMap<String, Vec<(&str, &str)>>, Box<dyn Error>> {
        let re = Regex::new(r"^((>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))\n)*(>[a-zA-Z0-9]+\n((([^;\n]+:[^;\n]+);)*([^;\n]+:[^\n;]+)))$").unwrap();
        if !re.is_match(content) {
            return Result::Err("Content is not proper formatted!")?;
        }

        let mut map: HashMap<String, Vec<(&str, &str)>> = HashMap::new();

        let entry_names: Vec<&str> = content.lines().filter(|x| x.starts_with(">")).collect();
        let lines: Vec<&str> = content.lines().collect();

        entry_names.into_iter().for_each(|name| {
            let idx = content.lines().position(|x| name == x).unwrap();
            let values: Vec<(&str, &str)> = lines
                .get(idx + 1)
                .unwrap()
                .split(";")
                .map(|key_value| {
                    let s: Vec<&str> = key_value.split(":").collect();
                    (s[0], s[1])
                })
                .collect();
            map.insert(name.replace(">", ""), values);
        });
        Result::Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Add;

    use crate::password_file::PasswordFile;

    #[test]
    fn parse_file() {
        let file_content =
            String::from(">Gmail\nusername:julianriegraf@gmail.com;password:1234567890\n")
                .add(">Darknet\nusername:blackHat666;password:pwd\n")
                .add(">Internet\nusername:sexyBienchen68;password:strongPassword123\n");
        let map = PasswordFile::parse_file_content(&file_content).unwrap();
        assert_eq!(map.len(), 3);

        assert!(map
            .get("Darknet")
            .unwrap()
            .contains(&("username", "blackHat666")));

        assert!(map.get("Darknet").unwrap().contains(&("password", "pwd")));

        assert!(map
            .get("Internet")
            .unwrap()
            .contains(&("username", "sexyBienchen68")));

        assert!(map
            .get("Internet")
            .unwrap()
            .contains(&("password", "strongPassword123")));
    }
}
