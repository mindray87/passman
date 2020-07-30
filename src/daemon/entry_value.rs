
#[derive(Debug, Clone, PartialEq)]
pub struct EntryValue {
    name: String,
    value: String
}

impl EntryValue {

    pub fn new(name: &str, value : &str) -> EntryValue {
        Self{
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    pub fn to_string(&self) -> String{
        format!("{}:{}", self.name, self.value)
    }

}