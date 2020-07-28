pub struct Password {
    pub password: String,
    pub account: String,
}

impl Password {
    pub fn save_pass(&self) {
        println!("Saving {} for {}", self.password, self.account);
    }
}
