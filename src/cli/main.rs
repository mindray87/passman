mod clipboard;
mod pass;
use std::env;
use std::net::TcpListener;
use std::net::TcpStream;

//terminal:
// passman verb option
// passman get gmail
// passman delete account
// passman get google -u -> username

fn main() {
    println!("passman starting...");
    start_connection();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print_help(),
        2 => {
            let cmd = &args[1];
            //let account_name = &args[2];

            match &cmd[..] {
                "help" => {
                    println!("help");
                    print_help();
                }
                _ => println!("b"),
            }
        }
        // passman command accountname
        3 | 4 | 5 => {
            let cmd = &args[1];
            let acc = &args[2];
            match &cmd[..] {
                "delete" => {
                    println!("del your account {}", acc);
                    //Todo: Send del request (acc) to daemon
                }
                "get" => {
                    println!("get {}", acc);
                    //Todo: send get request to daemon
                }
                "new" => {
                    println!("new {}", acc);
                    //Todo: args 4 there or create random
                    if args.len() >= 4 {
                        let username = &args[3];
                    } else {
                        println!("please enter the username u want to use:");
                        let mut username = String::new();

                        std::io::stdin()
                            .read_line(&mut username)
                            .expect("Failed to read line");

                        let username: &String = &username;
                        println!("your username is {}", username);
                    }
                    if args.len() >= 5 {
                        let password = &args[4];
                        println!("send to daemon");
                    } else {
                        println!(
                            "you have not entered a password. Should passman create it for you?"
                        );
                        let mut answer = String::new();
                        std::io::stdin()
                            .read_line(&mut answer)
                            .expect("Failed to read line");

                        let yes_no = &answer.trim();
                        println!("answer is : {}", yes_no);
                        match &yes_no[..] {
                            "y" | "Y" => {
                                println!("passman will generate your password now.");
                                println!(
                                    "How long do you want your password to be? (maximum of 128."
                                );
                                let mut pw_length = String::new();
                                std::io::stdin()
                                    .read_line(&mut pw_length)
                                    .expect("Failed to read line");

                                let pw_length: u32 =
                                    pw_length.trim().parse().expect("Please type a number!");
                                //TODO: ask for special chars
                                let password = make_pass(pw_length);
                                println!("generated: {}", password);
                            }
                            "n" | "N" => {
                                print_help();
                            }
                            _ => {
                                print_help();
                            }
                        }
                    }
                    //Todo: send create request to daemon
                }
                _ => print_help(),
            }
        }
        _ => print_help(),
    }
}

use rand::distributions::Alphanumeric;
use rand::Rng;
use std::io::Write;

fn start_connection() -> Result<TcpStream, &'static str> {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:34254") {
        Ok(stream)
    } else {
        println!("error");
        Err("could not establish connection to daemon.")
    }
}

pub fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, upercase lowercase, special chars !"§$%&...
    // concatinate lists as needed and than generate pass -> check afterwards
    let length = length as usize;
    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>();
    id
}

fn print_help() {
    println!("Usage options: passman <option>");
    println!("help -> display this.");
    println!();
    println!("get <accountname> -> this will search the db for your username and passwort for the given account");
    println!();
    println!("del <accountname> -> this will delete your username and password for <accountname> from the db if it exists");
    println!();
    println!("new <accountname> <username> <password>(optional) -> this will create an account in the db with your given username");
    println!("if you leave the password field empty, we'll ask you for your preferred password choices and generate it randomly.");
}
