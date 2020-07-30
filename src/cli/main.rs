//mod clipboard;
mod pass;
// extern crate clipboard;
// use clipboard::ClipboardContext;
// use clipboard::ClipboardProvider;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::net::{Shutdown, TcpListener};


use std::process::exit;
use std::str::from_utf8;
use std::time::Duration;
//CLIPBOARD ENDPOINT
fn main() {
    println!("passman starting...");
    let args: Vec<String> = env::args().collect();

    let acc: &String;
    let cmd;
    if args.len() > 1 {
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
                        "ADD {}\nusername:{};password:{}",
                        &acc, &username, &password
                    );
                    msg_daemon(data);
                }
                "close" => {
                    let data = format!("CLOSE\n'");
                    msg_daemon(data);
                }
                "create" | "open" => {
                    println!("debug 3 create open");
                    let filename = ask_for_filename();
                    let password = yes_or_no();
                    //password = &yes_no;
                    let data;
                    if cmd == "create" {
                        data = format!("CREATE {}\n{}", &filename, &password);
                    } else {
                        data = format!("OPEN {}\n{}", &filename, &password);
                    }
                    println!("data for daemon: {}", data);
                    msg_daemon(data);
                }
                "delete" => {
                    let acc = ask_for_accountname();
                    let data = format!("DELETE {}", &acc);
                    msg_daemon(data);
                }
                "get" => {
                    let acc = ask_for_accountname();

                    let data = format!("GET {}", &acc);
                    msg_daemon(data);
                }
                "help" => {
                    print_help();
                }
                _ => print_help(),
            }
        }
        // passman command accountname
        3 | 4 | 5 => {
            //open with param
            //close with param
            let cmd = &args[1];
            acc = &args[2];
            match &cmd[..] {
                "add" => {
                    let username;
                    if args.len() >= 4 {
                        username = &args[3];
                    } else {
                        tmp = ask_for_username();
                        username = &tmp;
                    }

                    if args.len() >= 5 {
                        password = &args[4]
                    } else {
                        yes_no = yes_or_no();
                        password = &yes_no;
                    }

                    let data = format!("ADD {}\nusername:{};password:{}", acc, username, password);
                    println!("data for daemon: {}", data);
                    msg_daemon(data);
                }
                "create" | "open" => {
                    println!("args len create open 3: {}", args.len());
                    if args.len() >= 3 {
                        filename = &args[2];
                    } else {
                        tmp = ask_for_filename();
                        filename = &tmp;
                    }
                    if args.len() >= 4 {
                        password = &args[3];
                    } else {
                        yes_no = yes_or_no();
                        password = &yes_no;
                    }
                    let data;
                    if cmd == "create" {
                        data = format!("CREATE {}\n{}", filename, password);
                    } else {
                        data = format!("OPEN {}\n{}", filename, password);
                    }
                    println!("data for daemon: {}", data);
                    msg_daemon(data);
                }
                "delete" => {
                    let data = format!("DELETE {}", acc);
                    msg_daemon(data);
                }
                "get" => {
                    let data = format!("GET {}", acc);
                    //let res = msg_daemon(data);
                    "Hallo Welt.".to_string();
                }
                _ => print_help(),
            }
        }
        _ => print_help(),
    }
}

/// Returns a u32 that is used to generate a random passwort with the given length.
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

/// Returns a String that contains the accountname.
fn ask_for_accountname() -> String {
    let mut input_username = String::new();

    println!("Please enter an account name for the password file");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your accountname is {}", input_username);
    input_username
}

/// Returns a String that contains the filename.
fn ask_for_filename() -> String {
    let mut input_username = String::new();

    println!("Please enter a name for your password file:");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your filename is {}", input_username);
    input_username
}
/// Returns a String that contains the username.
fn ask_for_username() -> String {
    let mut input_username = String::new();

    println!("Please enter the username you want to use:");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your username is {}", input_username);
    input_username
}

/// Returns a String that represents the servers response to our request.
///
/// # Arguments
///
/// *`data` - A String containing the request to the server.
///
/// # Examples
///
/// ```
/// let data: String = "ADD accountname:gmail;username:gmailuser;password:Gm4!L".to_string();
/// let resp = msg_daemon(data);
/// ```
fn msg_daemon(data: String) -> String {
    let mut tcp_stream = TcpStream::connect("localhost:7878").expect("Failed to connect.");
    println!(
        "Successfully connected to server {}",
        tcp_stream.peer_addr().unwrap().to_string()
    );
    tcp_stream.set_read_timeout(Some(Duration::new(3, 0)));
    tcp_stream.set_write_timeout(Some(Duration::new(3, 0)));

    let mut msg = data.as_bytes();
    tcp_stream.write_all(msg).unwrap();
    tcp_stream
        .shutdown(Shutdown::Write)
        .expect("Can not shutdown Write.");
    println!("Sent '{}', awaiting reply...", data);

    let mut buffer = BufReader::new(tcp_stream);
    let mut s = String::new();

    let response = match buffer.read_line(&mut s) {
        Ok(_) => {
            println!("Got reply: {}", s);
            s.to_string()
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
            e.to_string()
        }
    };
    response
}

/// Returns a randomly generated password string
///
/// #Arguments
///
/// * `length` - The chosen lenght for the password choosen by the user.
///
/// # Examples
///
/// ```
/// let length:u32 = 12;
/// make_pass(length);
/// ```
//TODO: Change arguments: uppercase: usize, lower: usize, digits: usize, specials: usize
fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, uppercase lowercase, special chars !"§$%&...
    // concatenate lists as needed and than generate pass -> check afterwards
    let upper = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>();
    let lower = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    // let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let special = vec![
    //     ,b', .', _', -', !', $', &', /', (', )', =', ?', #', *', +',
    //     <', >', %', (', )',
    // ];
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
    println!(
        "open <filename> -> opens the file <filename> with your stored accounts and passwords"
    );
    println!();
    println ! ("get <accountname> -> this will search the db for your username and passwort for the given account and copy the password to your clipboard");
    println!();
    println ! ("del <accountname> -> this will delete your username and password for <accountname> from the db if it exists");
    println!();
    println ! ("new <accountname> <username> <password>(optional) -> this will create an account in the db with your given username and password");
    println ! ("if you leave the password field empty, we'll ask you for your preferred password choices and generate it randomly.");
}

/// A Simple yes or no Question, in this case to check if the user wants to get a random password
/// or type a password, if not given as a param to passman
fn yes_or_no() -> String {
    println!("you have not entered a password. Should passman create it for you? (y / n)");
    let mut password_gen = String::from("");
    let mut answer = String::new();
    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let yes_no = answer.trim();
    match &yes_no[..] {
        "y" | "Y" | "yes" | "Yes" => {
            let len = ask_for_pass();
            password_gen = make_pass(len);
        }
        "n" | "N" | "no" | "No" => {
            println!("Please enter your costum password now: ");
            let mut answer = String::new();
            std::io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            password_gen = answer.trim().to_owned();
            if password_gen.len() <= 3 {
                println!("min passwordlength is 4");
                exit(1);
            }
            print_help();
        }
        _ => {
            print_help();
        }
    };
    password_gen
}
