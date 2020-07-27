mod pass;
mod clipboard;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;

//terminal:
// passman verb option
// passman get gmail
// passman delete account
// passman get google -u -> username

fn main() {
    println!("passman starting...");
    let stream = start_connection();
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
                },
                _ => println!("b"),
            }
        },
        // passman command accountname
        3 | 4 | 5 => {
            let cmd = &args[1];
            let acc = &args[2];
            let mut pw_length = String::new();
            let mut username = &String::new();
            let mut password = &String::new();
            let mut input_username = String::new();
            let mut password_gen = String::new();

            match &cmd[..] {
                "delete" => {
                    println!("del your account {}", acc);
                    //Todo: Send del request (acc) to daemon
                },
                "get" => {
                    println!("get {}", acc);
                    //Todo: send get request to daemon
                },
                "new" => {
                    println!("new {}", acc);
                    //Todo: args 4 there or create random
                    if args.len() >= 4 {
                        username = &args[3];
                    } else {
                        println!("please enter the username u want to use:");
                        std::io::stdin()
                            .read_line(&mut input_username)
                            .expect("Failed to read line");

                        username = &input_username;
                        println!("your username is {}", username);
                    }
                    if args.len() >= 5 {
                        password = &args[4];
                        println!("send to daemon");
                    } else {
                        println!("you have not entered a password. Should passman create it for you?");
                        let mut answer = String::new();
                        std::io::stdin()
                            .read_line(&mut answer)
                            .expect("Failed to read line");

                        let yes_no = &answer.trim();
                        println!("answer is : {}", yes_no);
                        match &yes_no[..] {
                            "y" | "Y" => {
                                println!("passman will generate your password now.");
                                println!("How long do you want your password to be? (maximum of 128.");

                                std::io::stdin()
                                    .read_line(&mut pw_length)
                                    .expect("Failed to read line");

                                let pw_length_no: u32 = pw_length.trim().parse().expect("Please type a number!");
                                //TODO: ask for special chars
                                password_gen = make_pass(pw_length_no);
                                password = &password_gen;
                                println!("generated: {}", password);
                            },
                            "n" | "N" => {
                                print_help();
                            },
                            _ => {
                                print_help();
                            }
                        }

                    }
                    //Todo: send create request to daemon
                    println!("to demon: account:{};username:{};password:{};", acc, username, password);
                    let data = format!("b'ADD {}\nusername:{};password:{};'", acc, username, password);
                    println!("data: {}", data);
                    //stream.write()
                },
                _ => print_help()
            }
        }
        _ => print_help()
    }



}


use rand::Rng;
use rand::distributions::Alphanumeric;
use std::io::Write;

fn start_connection() -> Result<TcpStream, &'static str> {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:34254"){
        println!("Connected to server.");
        Ok(stream)
    } else {
        println!("error");
        Err("could not establish connection to daemon.")
    }
}

//TODO: Change arguments: uppercase: usize, lower: usize, digits: usize, specials: usize
pub fn make_pass(length: u32) -> String {
    //TODO: make function that has 4 lists -> digits, upercase lowercase, special chars !"§$%&...
    // concatinate lists as needed and than generate pass -> check afterwards
    let upper = (b'A'..=b'Z')
        .map(|c| c as char)
        .collect::<Vec<_>>();
    let lower = (b'a'..=b'z')
        .map(|c| c as char)
        .collect::<Vec<_>>();
    let digits = vec![0,1,2,3,4,5,6,7,8,9];
    let special = vec![b',',b'.',b'_',b'-',b'!',b'$',b'&',b'/',b'(',b')',b'=',b'?',b'#',b'*',b'+',b'<',b'>',b'%',b'(',b')'];
    println!("digits {:#?}, uppercase: {:#?}, lowercase: {:#?}, special: {:#?}", digits, upper, lower, special);
    let length = length as usize;
    let id = rand::thread_rng().sample_iter(&Alphanumeric).take(length).collect::<String>();
    id
}

fn print_help(){
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