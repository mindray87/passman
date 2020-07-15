mod pass;
mod clipboard;


fn main() {
    let pass = pass::Password{
        account: "google".to_string(),
        password: "test".to_string()
    };
    pass.save_pass();

    println!("copy to clip");

    clipboard::clipboard::copy_to_clip(&pass);
    clipboard::clipboard::del_from_clip(&pass);

}
