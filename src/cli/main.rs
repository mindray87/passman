use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::net::TcpStream;
use std::process::exit;

use rand::thread_rng;

fn main() {


    let args: Vec<String> = env::args().collect();

    let acc: &String;
    let password;
    let filename;
    let yes_no;
    let tmp;
    match args.len() {
        0 | 1 => {
            print_help();
            //println!("debug 1");
        }
        2 => {
            //println!("debug 2");
            let cmd = &args[1];
            match &cmd[..] {
                "add" | "-a" => {
                    //println!("debug 4 add");
                    let acc = ask_for_accountname();
                    let username = ask_for_username();
                    let password = yes_or_no();
                    let data = format!(
                        "ADD {} username:{};password:{}",
                        &acc, &username, &password
                    );
                    println!("data: {}", data);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n New account added to your password file!\n");
                    } else {
                        print_error();
                    }
                }
                "close" | "-c" => {
                    let data = format!("CLOSE");
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n File closed!\n");
                    } else {
                        print_error();
                    }
                }
                "create" | "open" => {
                    //println!("debug 3 create open");
                    let filename = ask_for_filename();
                    let password: String;
                    //password = &yes_no;
                    let data;
                    if cmd == "create" {
                        password = make_pass(16);
                        data = format!("CREATE {} {}", &filename, &password);
                    } else {
                        password = yes_or_no();
                        data = format!("OPEN {} {}", &filename, &password);
                    }
                    //println!("data for daemon: {}", data);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n File ready!\n");
                    } else {
                        print_error();
                    }
                }
                "delete" => {
                    let acc = ask_for_accountname();
                    let data = format!("DELETE {}", &acc);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n Entry is deleted!\n");
                    } else {
                        print_error();
                        println!("\n You need to open the file you want to delete! ");
                    }
                }
                "print" | "-p" => print_command(&ask_for_accountname()),
                "help" | "-h" => {
                    print_help();
                }
                "get" | "-g" => {
                    let acc = ask_for_accountname();
                    let data = format!("CLIPBOARD {}", &acc);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!();
                        println!("############################################");
                        println!("#                                          #");
                        println!("#        The password is copied            #");
                        println!("#                to your                   #");
                        println!("#               Clipboard!                 #");
                        println!("#                                          #");
                        println!("############################################");
                        println!();
                    } else {
                        println!("\n Something went wrong");
                    }
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
                "add" | "-a" => {
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

                    let data = format!("ADD {} username:{};password:{}", acc, username, password);
                    //println!("data for daemon: {}", data);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n New account added to your password file!\n");
                    } else {
                        print_error();
                    }
                }
                "create" | "open" => {
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
                        data = format!("CREATE {} {}", filename, password);
                    } else {
                        data = format!("OPEN {} {}", filename, password);
                    }
                    //println!("data for daemon: {}", data);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n File ready!\n");
                    } else {
                        print_error();
                    }
                }
                "get" | "-g" => {
                    let data = format!("CLIPBOARD {}", acc);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!();
                        println!("############################################");
                        println!("#                                          #");
                        println!("#        The password is copied            #");
                        println!("#                to your                   #");
                        println!("#               Clipboard!                 #");
                        println!("#                                          #");
                        println!("############################################");
                        println!();
                    } else {
                        print_error();
                    }
                }
                "delete" => {
                    let data = format!("DELETE {}", &acc);
                    let res = msg_daemon(data);
                    if res == "OK" {
                        println!("\n Entry is deleted!\n");
                    } else {
                        print_error();
                        println!("\n You need to open the file you want to delete! ");
                    }
                }
                "print" | "-p" => print_command(acc),
                _ => print_help(),
            }
        }
        _ => print_help(),
    }
}

fn print_command(acc : &String) {
    let data = format!("GET {}", &acc);
    let res = msg_daemon(data);

    if res.starts_with("ERR NotFound") {
        println!("There is no entry for '{}'.", acc);
    } else {
        let res = res.split(";").nth(1).unwrap().split(":").nth(1).unwrap();

        println!();
        println!("############################################");
        println!("#                                          ");
        println!("#        The password for {} is:           ", acc);
        println!("#                {}                        ", res);
        println!("#                                          ");
        println!("############################################");
        println!();
    }
}

/// Returns a u32, that is used to evaluate the wanted password length.
fn ask_for_pass() -> u32 {
    println!("\nPassman will generate your password now.");
    println!("How long do you want your password to be? (maximum of 128.)\n");
    let mut pw_length = String::new();
    std::io::stdin()
        .read_line(&mut pw_length)
        .expect("Failed to read line");

    let pw_length: u32 = pw_length.trim().parse().expect("Please type a number!");
    pw_length
}

/// Returns a string containing the accountname
fn ask_for_accountname() -> String {
    let mut input_username = String::new();
    println!("\nPlease enter an account name for the password file\n");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");
    input_username.trim().to_string()
}

/// Returns a string containing the passwordfiles filename
fn ask_for_filename() -> String {
    let mut input_username = String::new();

    println!("\nPlease enter a name for your password file:\n");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your filename is {}", input_username);
    input_username.trim().to_string()
}

/// Returns a string containing the username
fn ask_for_username() -> String {
    let mut input_username = String::new();

    println!("Please enter the username you want to use: \n");
    std::io::stdin()
        .read_line(&mut input_username)
        .expect("Failed to read line");

    println!("your username is {}", input_username);
    input_username.trim().to_string()
}

/// Returns a String, that contains the servers response.
///
/// # Arguments
///
/// *`request` - a String that contains the request to the server.
///
/// # Examples
///
/// ```
/// let request = "ADD gmail\nusername:user;password:1234".to_string();
/// let resp = msg_daemon(request);
/// assert_eq!(resp, "Ok".to_string());
/// ```
fn msg_daemon(request: String) -> String {
    let mut tcp_stream = match TcpStream::connect("localhost:7878") {
        Ok(s) => s,
        Err(_) => return "Can not connect to Daemon. Is it running?".to_string(),
    };
    // tcp_stream.set_read_timeout(Some(Duration::new(3, 0)));
    // tcp_stream.set_write_timeout(Some(Duration::new(3, 0)));

    let msg = request.as_bytes();
    tcp_stream.write_all(msg).unwrap();
    tcp_stream
        .shutdown(Shutdown::Write)
        .expect("Can not shutdown Write.");

    let mut buffer = BufReader::new(tcp_stream);
    let mut s = String::new();

    match buffer.read_line(&mut s) {
        Ok(_) => { s.to_string() }
        Err(e) => { e.to_string() }
    }
}

/// Returns a randomly generated password string
///
/// # Arguments
///
/// * `length` - The chosen lenght for the password choosen by the user.
///
/// # Examples
///
/// ```
/// let pass = make_pass(12);
/// ```
fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, uppercase lowercase, special chars !"§$%&...
    // concatenate lists as needed and than generate pass -> check afterwards
    use rand::seq::SliceRandom;
    let upper = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<_>>();
    let lower = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let special = vec![
        ',', '.', '_', '-', '!', '$', '&', '/', '(', ')', '=', '?', '#', '*', '+', '<', '>', '%',
        '~',
    ];

    let char_list: Vec<char>;

    println!("Pls choose the set of characters for your password: ");
    println!();
    println!("\t Uppercase and lowercase alphabetical only: \t\t[1]");
    println!("\t Alphabet and digits 0-10: \t\t\t\t[2]");
    println!("\t Include special characters (e.g. #,.;:*()): \t\t[3]");
    println!();
    println!("selecting 3 gives you the maximum safety level and is set by default \n");

    let mut answer = String::new();

    std::io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let answer = answer.trim();
    let answer: u32 = match answer.trim().parse() {
        Ok(num) => {
            println!("num: {}", num);
            num
        }
        Err(_) => 3,
    };

    if answer == 2 {
        char_list = upper
            .iter()
            .cloned()
            .chain(lower.iter().cloned())
            .chain(digits.iter().cloned())
            .collect();
    } else if answer == 1 {
        char_list = upper
            .iter()
            .cloned()
            .chain(lower.iter().cloned())
            .collect();
    } else {
        char_list = upper
            .iter()
            .cloned()
            .chain(lower.iter().cloned())
            .chain(digits.iter().cloned())
            .chain(special.iter().cloned())
            .collect();
    }

    let length = length as usize;
    let mut pass: Vec<char>;
    loop {
        pass = char_list
            .choose_multiple(&mut thread_rng(), length)
            .into_iter()
            .cloned()
            .collect();
        if answer == 1 {
            if pass.iter().find(|x| upper.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| lower.contains(x)).is_none() {
                continue;
            } else {
                break;
            }
        } else if answer == 2 {
            if pass.iter().find(|x| upper.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| lower.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| digits.contains(x)).is_none() {
                continue;
            } else {
                break;
            }
        } else {
            if pass.iter().find(|x| upper.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| lower.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| digits.contains(x)).is_none() {
                continue;
            }
            if pass.iter().find(|x| special.contains(x)).is_none() {
                continue;
            } else {
                break;
            }
        }
    }

    let mut pwd = String::new();
    pwd += &pass.into_iter().collect::<String>();
    pwd
}

fn print_error() {
    println!();
    println!("############################################");
    println!("#                                          #");
    println!("#                                          #");
    println!("#        Starting passman                  #");
    println!("#              (\\./)                       #");
    println!("#              (-.-)                       #");
    println!("#              (.)(.)                      #");
    println!("#                                          #");
    println!("#       Something went wrong               #");
    println!("#                                          #");
    println!("############################################");
    println!();
}

/// Prints a help message if user inputs invalid commands
fn print_help() {
    println!("\nname: passman\nversion: 1.0\nauthors: Julian Riegraf, Patrick Toth\nabout: Simple cli password managment tool");
    println!("\nUsage");
    println!("\tpassman [OPTIONS] <INPUT>\n");
    println!("FLAGS:\n");
    println!("\thelp, -h \t\tPrints this message\n");
    println!("OPTIONS");
    println!("\t-a, add <ACCOUNTNAME> <USERNAME> <PASSWORD> \tAdd an account to the password file. If you leave Input fields empty, passman will ask you to provide them.");
    println!("\t-g, get <ACCOUNTNAME> \t\t\t\tGet the password for the given account <ACCOUNTNAME>. The password will be copied to your clipboard for thirty seconds.");
    println!("\t-p, print <ACCOUNTNAME> \t\t\tPrints the password of account <ACCOUNTNAME> to your terminal.\n");
    println!("\tcreate <FILENAME> <MASTERPASSWORD> \t\tCreate a new password file with a master password.");
    println!("\topen <FILENAME> <MASTERPASSWORD> \t\tOpen the password file <FILENAME> with the password.");
    println!("\tdelete <FILENAME> <MASTERPASSWORD> \t\tDelete the password file <FILENAME>. You have to open it before you can delete it.\n");
}

/// Returns a String that contains the user password.
fn yes_or_no() -> String {
    println!("you have not entered a password. Should passman create it for you? (y / n)");
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
