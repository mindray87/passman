mod pass;
mod clipboard;

use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::io::{BufReader, BufWriter, Write, BufRead};

use rand::Rng;
use rand::distributions::Alphanumeric;
//terminal:
// passman verb option
// passman get gmail
// passman delete account
// passman get google -u -> username

fn main() {
    println!("passman starting...");
    let args: Vec<String> = env::args().collect();

    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("connected to server");
            match args.len() {
                1 => print_help(),
                2 => {
                    let cmd = &args[1];

                    match &cmd[..] {
                        "help" => {
                            print_help();
                        },
                        _ => println!("b"),
                    }
                }
                // passman command accountname
                3 | 4 | 5 => {
                    let cmd = &args[1];
                    let acc = &args[2];
                    let mut username = &String::new();
                    let mut password = &String::new();
                    let tmp;
                    let yes_no: String;

                    match &cmd[..] {
                        "delete" => {
                            let data = format!("b'DELETE {}'", acc);
                            msg_daemon(data, stream);
                        }
                        "get" => {
                            println!("get {}", acc);
                            let data = format!("b'GET {}'", acc);
                            msg_daemon(data,stream);
                        }
                        "new" => {
                            match args.len() {
                                3 => {
                                    // no username no pass
                                    tmp = ask_for_username();
                                    username = &tmp;
                                    yes_no = yes_or_no();
                                    password = &yes_no;
                                },
                                4 => {
                                    username = &args[3];
                                    //ask for password generator
                                    yes_no = yes_or_no();
                                    password = &yes_no;

                                },
                                5 => {
                                    password = &args[4];
                                },
                                _ => print_help()
                            }
                            let data = format!("b'ADD {}\nusername:{};password:{};'", acc, username, password);
                            println!("data: {:#?}", data);
                            msg_daemon(data, stream);
                        }
                        _ => print_help()
                    }
                }
                _ => print_help()
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn ask_for_pass() -> u32{
    println!("passman will generate your password now.");
    println!("How long do you want your password to be? (maximum of 128.");
    let mut pw_length = String::new();

    std::io::stdin()
        .read_line(&mut pw_length)
        .expect("Failed to read line");

    let pw_length: u32 = pw_length.trim().parse().expect("Please type a number!");
    pw_length
}

fn ask_for_username() -> String {
    let mut input_username = String::new();

    println!("Please enter the username you want to use:");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your username is {}", input_username);
    input_username
}

fn msg_daemon(data: String, stream: TcpStream) {
    let mut writer = BufWriter::new(stream);
    writer.write(data.as_bytes()).expect("could not write");
}


/// Returns a randomly generated password string
///
/// #Arguments
///
/// * `length` - The chosen lenght for the password choosen by the user.
//TODO: Change arguments: uppercase: usize, lower: usize, digits: usize, specials: usize
pub fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, uppercase lowercase, special chars !"§$%&...
    // concatenate lists as needed and than generate pass -> check afterwards
    let upper = (b'A'..=b'Z')
        .map(|c| c as char)
        .collect::<Vec<_>>();
    let lower = (b'a'..=b'z')
        .map(|c| c as char)
        .collect::<Vec<_>>();
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let special = vec![b',', b'.', b'_', b'-', b'!', b'$', b'&', b'/', b'(', b')', b'=', b'?', b'#', b'*', b'+', b'<', b'>', b'%', b'(', b')'];
    println!("digits {:#?}, uppercase: {:#?}, lowercase: {:#?}, special: {:#?}", digits, upper, lower, special);
    let length = length as usize;
    let id = rand::thread_rng().sample_iter(&Alphanumeric).take(length).collect::<String>();
    id
}

fn print_help() {
    println!("Usage options: passman <option>");
    println!();
    println!("help -> display this.");
    println!();
    println!("get <accountname> -> this will search the db for your username and passwort for the given account");
    println!();
    println!("del <accountname> -> this will delete your username and password for <accountname> from the db if it exists");
    println!();
    println!("new <accountname> <username> <password>(optional) -> this will create an account in the db with your given username");
    println!("if you leave the password field empty, we'll ask you for your preferred password choices and generate it randomly.");
}

fn yes_or_no() -> String{
    println!("you have not entered a password. Should passman create it for you?");
    let mut password_gen= String::from("");
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let yes_no = answer.trim();
    match &yes_no[..] {
        "y" | "Y" => {
            let len = ask_for_pass();
            password_gen = make_pass(len);
        }
        "n" | "N" => {
            print_help();
        }
        _ => {
            print_help();
        }
    };
    password_gen

}