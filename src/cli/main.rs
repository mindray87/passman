mod clipboard;
mod pass;

use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write, Read};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use rand::distributions::Alphanumeric;
use rand::Rng;
//terminal:
// passman verb option
// passman get gmail
// passman delete account
// passman get google -u -> username
extern crate clipboard as other_clip;

use other_clip::ClipboardProvider;
use other_clip::ClipboardContext;




fn main() {

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let the_string = "Hello, world!";

    ctx.set_contents(the_string.to_owned()).unwrap();

    let mut abc = ctx.get_contents().unwrap();
    println!("{}", abc);

    let the_other_one = "Hello, World";
    ctx.set_contents(the_other_one.to_owned()).unwrap();
    let mut abc = ctx.get_contents().unwrap();

    println!("{}", abc);

    println!("passman starting...");
    let args: Vec<String> = env::args().collect();

    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("connected to server");
            let acc: &String;
            let cmd;
            if args.len() > 1{
                cmd = &args[1];
            }
            let password;
            let filename;
            let yes_no;
            let tmp;
            match args.len() {
                0 | 1 => {
                    print_help();
                    println!("debug 1");
                }
                2 => {
                    println!("debug 2");
                    let cmd = &args[1];
                    match &cmd[..] {
                        "add" => {
                            println!("debug 4 add");
                            let acc = ask_for_accountname();
                            let username = ask_for_username();
                            let password = yes_or_no();
                            let data = format!(
                                "b'ADD {}\nusername:{};password:{};",
                                &acc, &username, &password
                            );
                            msg_daemon(data, stream);
                        },
                        "close" => {
                            let data = format!("b'CLOSE\n'");
                            msg_daemon(data, stream);
                        }
                        "create" | "open" => {
                            println!("debug 3 create open");
                            let filename = ask_for_filename();
                            let password = yes_or_no();
                            //password = &yes_no;
                            let data;
                            if cmd == "create" {
                                data = format!("b'CREATE {}\n{};'", &filename, &password);
                            } else {
                                data = format!("b'OPEN {}\n{};'", &filename, &password);
                            }
                            println!("data for daemon: {}", data);
                            msg_daemon(data, stream);
                        },
                        "delete" => {
                            let acc = ask_for_accountname();
                            let data = format!("b'DELETE {}'", &acc);
                            msg_daemon(data, stream);
                        }
                        "get" => {
                            let acc = ask_for_accountname();

                            let data = format!("b'GET {}'", &acc);
                            msg_daemon(data, stream);
                        },
                        "help" => {
                            print_help();
                        },
                        _ => println!("b"),
                    }
                }
                // passman command accountname
                3 | 4 | 5 => {
                    let cmd = &args[1];
                    acc = &args[2];
                    match &cmd[..] {
                        "add" => {
                            let username;
                            if args.len() > 3{
                                username = &args[3];
                            } else {
                                tmp = ask_for_username();
                                username = &tmp;
                            }

                            if args.len() > 4 {
                                password = &args[4]
                            } else {
                                yes_no = yes_or_no();
                                password = &yes_no;
                            }

                            let data = format!(
                                "b'ADD {}\nusername:{};password:{};",
                                acc, username, password
                            );
                            println!("data for daemon: {}", data);
                            msg_daemon(data, stream);
                        },
                        "create" | "open" => {
                            if args.len() > 3{
                                filename = &args[3];
                            } else {
                                tmp = ask_for_filename();
                                filename = &tmp;
                            }
                            if args.len() > 4{
                                password = &args[4];
                            } else {
                                yes_no = yes_or_no();
                                password = &yes_no;
                            }
                            let data;
                            if cmd == "create" {
                                data = format!("b'CREATE {}\n{};'", filename, password);
                            } else {
                                data = format!("b'OPEN {}\n{};'", filename, password);
                            }
                            println!("data for daemon: {}", data);
                            msg_daemon(data, stream);
                        }
                        "delete" => {
                            let data = format!("b'DELETE {}'", acc);
                            msg_daemon(data, stream);
                        },
                        "get" => {
                            let data = format!("b'GET {}'", acc);
                            let res = msg_daemon(data,stream);
                            create_clipboard(res);

                        },
                        _ => print_help()
                    }
                }
                _ => print_help(),
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}



fn ask_for_userinput(option: String) {
//...TODO
}

fn ask_for_pass() -> u32 {
    println!("passman will generate your password now.");
    println!("How long do you want your password to be? (maximum of 128.");
    let mut pw_length = String::new();

    std::io::stdin()
        .read_line(&mut pw_length)
        .expect("Failed to read line");

    let pw_length: u32 = pw_length.trim().parse().expect("Please type a number!");
    pw_length
}

fn ask_for_accountname() -> String {
    let mut input_username = String::new();

    println!("Please enter an account name for the password file");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your accountname is {}", input_username);
    input_username
}

fn ask_for_filename() -> String {
    let mut input_username = String::new();

    println!("Please enter a name for your password file:");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your filename is {}", input_username);
    input_username
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

fn create_clipboard(context: String) {
    let clp_thread = thread::spawn(move || {
        ctx.set_contents(res.to_owned().unwrap());
    });
}

fn msg_daemon(data: String, mut stream: TcpStream) -> String{
    let mut writer = BufWriter::new(&stream);
    writer.write(data.as_bytes()).expect("could not write");
    writer.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_to_string(&mut response).expect("could not read");
    println!("Server received {:#?}", response);
    response
}

/// Returns a randomly generated password string
///
/// #Arguments
///
/// * `length` - The chosen lenght for the password choosen by the user.
//TODO: Change arguments: uppercase: usize, lower: usize, digits: usize, specials: usize
fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, uppercase lowercase, special chars !"§$%&...
    // concatenate lists as needed and than generate pass -> check afterwards
    let upper = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>();
    let lower = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let special = vec![
        b',', b'.', b'_', b'-', b'!', b'$', b'&', b'/', b'(', b')', b'=', b'?', b'#', b'*', b'+',
        b'<', b'>', b'%', b'(', b')',
    ];
    let length = length as usize;
    let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect::<String>();
    id
}

/// Prints a help message if user inputs invalid commands
fn print_help() {
    println!("Usage options: passman <option>");
    println!();
    println!("help -> list of usable commands:");
    println!("first create or open your database with the create/open command. than do whatever you like to do :)");
    println!();
    println!("create <filename> -> creates an encrypted file, with your accounts and passwords in the <filename> file");
    println!();
    println!("open <filename> -> opens the file <filename> with your stored accounts and passwords");
    println!();
    println ! ("get <accountname> -> this will search the db for your username and passwort for the given account and copy the password to your clipboard");
    println!();
    println ! ("del <accountname> -> this will delete your username and password for <accountname> from the db if it exists");
    println!();
    println ! ("new <accountname> <username> <password>(optional) -> this will create an account in the db with your given username and password");
    println ! ("if you leave the password field empty, we'll ask you for your preferred password choices and generate it randomly.");
}

fn yes_or_no() -> String {
    println!("you have not entered a password. Should passman create it for you?");
    let mut password_gen = String::from("");
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
