pub struct Password {
    pub password: String,
    pub account: String,
}

impl Password {
    pub fn save_pass(&self) {
        println!("Saving {} for {}", self.password, self.account);
    }
}


// match args.len() {
// 1 => {
// print_help();
// println!("debug 1");
// },
// 2 => {
// println!("debug 2");
// let cmd = &args[1];
// match &cmd[..] {
// "help" => {
// print_help();
// },
// // passman create filename password (0, 1, 2, 3)
// "create" | "open" => {
// match args.len() {
// 2 => {
// tmp = ask_for_filename();
// filename = &tmp;
// yes_no = yes_or_no();
// password = &yes_no;
// },
// 3 => {
// filename = &args[3];
// yes_no = yes_or_no();
// password = &yes_no;
// },
// 4 => {
// password = &args[4];
// },
// _ => print_help()
// }
// let data;
// if cmd == "create"{
// data = format!("b'CREATE {}\n{};'", filename, password);
// } else {
// data = format!("b'OPEN {}\n{};'", filename, password);
// }
// println!("data: {}", data);
// msg_daemon(data, stream);
// },
// "add" => {
// match args.len() {
// 3 => {
// // no username no pass
// acc = &args[2];
// tmp = ask_for_username();
// username = &tmp;
// yes_no = yes_or_no();
// password = &yes_no;
// },
// 4 => {
// username = &args[3];
// //ask for password generator
// yes_no = yes_or_no();
// password = &yes_no;
//
// },
// 5 => {
// password = &args[4];
// },
// _ => print_help()
// }
// let data = format!("b'ADD {}\nusername:{};password:{};'", acc, username, password);
// println!("data: {:#?}", data);
// msg_daemon(data, stream);
// }
// _ => println!("b"),
// }
// }
// // passman command accountname
// 3 | 4 | 5 => {
//
//     //create -> create new File
//     //add -> add pass to file
//     //open
//     //close
//     match &cmd[..] {
//         "add" => {
//
//         },
//         "delete" => {
//             let data = format!("b'DELETE {}'", acc);
//             msg_daemon(data, stream);
//         },
//         "get" => {
//             println!("get {}", acc);
//             let data = format!("b'GET {}'", acc);
//             msg_daemon(data,stream);
//         },
//         _ => print_help()
//     }
// }
// _ => print_help()
// }