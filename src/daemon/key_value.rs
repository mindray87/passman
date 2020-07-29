
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue{
    key: String,
    value: String
}

impl KeyValue{

    pub fn new(key: &str, value : &str) -> KeyValue{
        Self{
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn to_str(&self) -> String{
        format!("{}:{}", self.key, self.value)
    }

}