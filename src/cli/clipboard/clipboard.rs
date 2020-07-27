use crate::pass::Password;

extern crate copypasta;

pub fn copy_to_clip(password: &Password){
    println!("copied password {}", password.password);
    del_from_clip(&password);
}

pub fn del_from_clip(password: &Password){
    println!("del pass from clip {}",password.password);

}