#![allow(unused_imports, dead_code, unused_must_use, unused, unknown_lints)]
use std::io::{self, Read};
use std::fs::File;

fn main() {
    let mut loops = 2;
    let mut uname = String::new();
    let mut upassd = String::new();
    let file_creds = "./nothing.key";
    let mut raw_creds = String::new();

    struct cred_struct {
       uname: String, 
        passd: String
    };

    let mut f = File::open(file_creds).expect("Unable to read credentials file.");
    
    f.read_to_string(&mut raw_creds);
    let mut data = raw_creds.lines();
    let gCreds = {cred_struct { uname: String::from(data.next().unwrap()), passd: String::from(data.next().unwrap())}};

    println!("{}",gCreds.passd);

//    while loops < 3 {
//        println!("Enter username:\n");
//        io::stdin().read_line(&mut uname);
//        println!("Enter password:\n");
//        io::stdin().read_line(&mut upassd);
//        println!("Username input: {}", uname);
//        println!("Password input: {}", upassd);

//        loops = loops + 1;
//    }
}
