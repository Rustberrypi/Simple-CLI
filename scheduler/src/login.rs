#![allow(unused_imports, dead_code, unused_must_use, unused, unknown_lints, non_camel_case_types, non_snake_case)]
use std::io::{self, Read};
use std::fs::File;
extern crate rpassword;

pub fn login() -> bool {
    let mut loops = 0;
    let mut uname = String::new();
    let mut upassd = String::new();
    let file_creds = "src/.nothing.key";
    let mut raw_creds = String::new();
    let mut returnVal = false;

    struct cred_struct {
       uname: String, 
       passd: String
    };

    let mut f = File::open(file_creds).expect("Unable to read credentials file.");
    
    
    f.read_to_string(&mut raw_creds);
    let mut data = raw_creds.lines();
    let gCreds  = {cred_struct { uname: String::from(data.next().unwrap()), passd: String::from(data.next().unwrap()) }};
    let okCreds = {cred_struct { uname: String::from(data.next().unwrap()), passd: String::from(data.next().unwrap()) }}; 

    println!("Good Creds: {}, {}", gCreds.uname, gCreds.passd);
    println!("Ok Creds: {}, {}", okCreds.uname, okCreds.passd);

    while loops < 3 {
        println!("{}", returnVal);
        println!("Enter username:");
        io::stdin().read_line(&mut uname);

        if uname.trim().eq(&gCreds.uname) {
           upassd = rpassword::prompt_password_stdout("Password: ").unwrap().to_string();
           if upassd.eq(&gCreds.passd) {
              loops = 3;
              returnVal = true;
           } else {
              loops = loops + 1;
              uname = String::new();
              upassd = String::new();
           }
        } else if uname.trim().eq(&okCreds.uname) {
           upassd = rpassword::prompt_password_stdout("Password: ").unwrap().to_string();
           if upassd.eq(&okCreds.passd) {
              loops = 3;
           } else {
              loops = loops + 1;
           }
        } else {
           loops = loops + 1;
           uname = String::new();
        }
    }
    returnVal
}