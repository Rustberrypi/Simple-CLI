extern crate argon2;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use rand::RngCore;
use rand::rngs::OsRng;

const KEY_FILE: &str = "./.nothing.key";
const MAX_ENTRY_LENGTH: usize = 64;

pub struct UserCred {
	name: [u8; MAX_ENTRY_LENGTH],
	access: u8,
	salt: [u8; 16],
	pash: [u8; MAX_ENTRY_LENGTH],
}

impl UserCred {
	fn new() -> Result<UserCred, Box<dyn Error>> {
		// TODO NEW WOO WOO

		// Standard input stream, Hasher config, temporary variables
		let inpt = io::stdin();
		let mut out = io::stdout();
		let config = argon2::Config::default();
		let mut tmp1 = String::new();
		let mut tmp2 = String::new();
		let mut okay: bool = false;
			let mut new_user = UserCred {
			name: String::new(),
			access: 2,
			salt: [0u8; 16],
			pash: String::new(),
		};
	
		// Get a name to use; validate for no whitespace
		println!("Enter a user name.  Names are case-sensitive, and may not contain whitespace.");
		while !okay {
			okay = true;
			print!("Name: ");
			out.flush()?;
			inpt.read_line(&mut tmp1)?;
			tmp1 = String::from(tmp1.trim());
			
			println!("You entered \"{}\".", &tmp1);
			for c in tmp1.chars() {
				if c.is_ascii_whitespace() || tmp1.len() > MAX_ENTRY_LENGTH { okay = false; }
			}
			if !okay { 
				println!("User names may not contain spaces, tabs, or newlines.");
				tmp1.clear();
			}
		}
		new_user.name = tmp1.as_bytes();
		okay = false;
		tmp1.clear();
		
		// Get a password to use; validate by double-checking
		println!("Please choose a password.  Passwords are case-sensitive.");
		println!("For security, the password you type will not appear on screen;");
		println!("You will be asked to type it again to confirm.");
		while !okay {
			okay = true;
			print!("Password: ");
			out.flush()?;
			inpt.read_line(&mut tmp1)?;
			tmp1 = String::from(tmp1.trim());
			print!("Password (again): ");
			out.flush()?;
			inpt.read_line(&mut tmp2)?;
			tmp2 = String::from(tmp2.trim());
			if tmp1 != tmp2 || tmp1.len() > MAX_ENTRY_LENGTH { okay = false; }
			if !okay {
				println!("Your first and second entries did not match.  Try again.");
				tmp1.clear();
				tmp2.clear();
			}
		}
		println!("Please remember your password, as it cannot be recovered.");
		
		// Set read/write access
		okay = false;
		while !okay {
			println!("Set user access level:");
			println!("    1) Read-Only");
			println!("    2) Read & Edit");
			print!(" > ");
			out.flush()?;
			inpt.read_line(&mut tmp1)?;
			tmp1 = String::from(tmp1.trim());
			let num: u8 = tmp1.parse().expect("Valid access levels are 1 and 2.");
			if (num < 1) || (num > 2) { okay = false; }
			if !okay {
				println!("Valid access levels are 1 and 2.");
			}
		}
		
		// Get a salt value;  flushing out the first value produced by OsRng
		// is recommended when using it early in the boot sequence; obviously
		// that isn't the case here, but it should have been, so I'm including it.
		let mut peanuts = [0u8; 16];
		OsRng.fill_bytes(&mut peanuts);
		peanuts = [0u8; 16];
		OsRng.fill_bytes(&mut peanuts);
		
		new_user.salt = peanuts;
		
		// Hash the salted password string using the Argon2 algorithm
		let hash = argon2::hash_encoded(tmp1.as_bytes(), &peanuts, &config)?;
		new_user.pash = hash.split_at(28).1;
		
		Ok(new_user)
	}
	
	pub fn add_user(usr: UserCred) -> Result<bool, Box<dyn Error>> {
		// TODO: Add a user to the key file
		// Open key file
		let key = File::open(KEY_FILE)?;
		let mut reader = BufReader::new(&key);
		let mut writer = BufWriter::new(&key);
		let mut contents = String::new();
		
		let mut to_add: Vec<u8> = Vec::new();
		for b in usr.name.as_bytes().iter() {
			to_add.push(*b);
		}
		','.encode_utf8(&mut to_add);
		to_add.push(usr.access);
		','.encode_utf8(&mut to_add);
		for b in usr.salt.iter() {
			to_add.push(*b);
		}
		','.encode_utf8(&mut to_add);
		for b in usr.pash.as_bytes().iter() {
			to_add.push(*b);
		}
		
		writer.write(&to_add)?;
		writer.flush()?;
		
		// Read key file into a vector; may not need this for "add"
		reader.read_to_string(&mut contents)?;
		let extant_creds: Vec<&str> = contents.split('\n').collect();
		
		Ok(true)
	}
	
	pub fn verify(name: String, password: String) -> Result<bool, Box<dyn Error>> {
		// TODO CHECKIT
		// Open input stream
		// let config = argon2::Config::default();
		let key = File::open(KEY_FILE)?;
		let mut reader = BufReader::new(key);
		let mut contents = String::new();
		
		// Read key file into a vector
		reader.read_to_string(&mut contents)?;
		let extant_creds: Vec<&str> = contents.split('\n').collect();
		
		// DEBUG SCAFFOLDING; REMOVE FOR PRODUCTION
		println!("There are currently {} sets of user credentials", extant_creds.len());
		for usr in extant_creds {
			println!("{}", usr);
		}
		
		for cred in &extant_creds {
			let fields: Vec<String> = vec!(cred.split(',').collect()?);
			if name = fields.get(0)? {
				if argon2::verify_encoded(&password, &fields.get(2)?.as_bytes()).unwrap() {
					if fields.get(1)? = 1 {
						Ok(false);
					} else {
						Ok(true);
					}
				}
			}
		}
		Err(String::from("Unable to verify username/password combination."));

	}
}