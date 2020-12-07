#![allow(unused_imports, dead_code, unused_must_use, unused, unknown_lints, non_camel_case_types, non_snake_case)]
use std::io::{self, Read};
use std::any::type_name;
use std::error::Error;
use std::fs::File;
use std::option::Option;
use crate::user_credentials;
extern crate rpassword;
extern crate argon2;

pub fn login() -> Result<bool, Box<dyn Error, >> {
    let mut loops = 0;
    let mut uname = String::new();
    let mut upassd = String::new();
    let mut raw_creds = String::new();
    let mut returnVal: Option<bool> = None;

   // let mut creds = user_credentials::turn_key().expect("Couldn't open the key!");
    let config = argon2::Config::default();
    while loops < 3 {
       let mut creds = user_credentials::turn_key().expect("Couldn't open the key!");
       println!("Creds contains {} element(s).", creds.len());
       println!("Here, let me print them for you.");
       for c in &creds {
         println!("Here's a cred: {}", c);
      }

       println!("Enter username:");
        io::stdin().read_line(&mut uname);
        if uname.trim().eq("config") {
            let user = user_credentials::UserCred::new().expect("NO!");
            creds.push(user);
            user_credentials::update_key(creds);
        } else {
         upassd = rpassword::prompt_password_stdout("Password: ").unwrap().to_string();
         // for user in &creds {
         //    if user.name().eq(&uname) {
         //       let hash = argon2::hash_encoded(upassd.as_bytes(), &user.salt()[..], &config).unwrap();
         //       if hash.split_at(28).1.as_bytes() == &user.pash()[..] {
         //          returnVal = Some(user.access() == 2);
         //          loops = 3;
         //       }
         //    }
         // }
         returnVal = user_credentials::verify_user(creds, &uname, &upassd).ok();
      }
      loops = loops + 1;
    }
    
    match returnVal {
       Some(a) => Ok(a),
       None => Err("Error".into()),
    }
}

// fn type_of(_: T) -> &'static str {
//    type_name::()
// }