extern crate argon2;
extern crate rpassword;
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use rand::RngCore;
use rand::rngs::OsRng;

const KEY_FILE: &str = "./.nothing.key";
const LARGE_BLOCK_SIZE: usize = 64;
const SMALL_BLOCK_SIZE: usize=16;

pub struct UserCred {
	name: Vec<u8>,
	access: u8,
	salt: Vec<u8>,
	pash: Vec<u8>,
}

// TODO: Create functions to translate this enum to & from single bytes
pub enum AccessLevel {
	ReadOnly,
	ReadWrite,
}

impl fmt::Display for UserCred {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}{}{}",
				String::from_utf8_lossy(&self.name),
				self.access.to_string(),
				String::from_utf8_lossy(&self.salt),
				String::from_utf8_lossy(&self.pash),
			)
	}
}

pub fn turn_key() -> Result<Vec<UserCred>, &'static str> {
	const ARGON: &str = "$argon2i$v=19$m=4096,t=3,p=1"; // 28 bytes
	let key = File::open(KEY_FILE).expect("Unable to open key file!");
	let mut reader = BufReader::new(key);
	let mut contents: Vec<u8> = Vec::new();
	// let mut debug: String = String::new();
	// reader.read_to_string(&mut debug).expect("nawp");
	// println!("{}", debug);
	// eader = BufReader::new(File::open(KEY_FILE).expect("Bananas Foster"));
	reader.read_to_end(&mut contents).expect("Unexpected failure reading key file.");
	println!("There are now {} elements in 'contents'., and {} elements in the reader's buffer.", contents.len(), reader.buffer().len());
	let keylib = parse_key(contents);
	Ok(keylib)
}

fn parse_key(inpt: Vec<u8>) -> Vec<UserCred> {
	if inpt.len() < 145 {
		return Vec::new()
	}
	let mut users: Vec<UserCred> = vec!();
	let mut i: usize = 0;		
	while i < inpt.len() {
		// TODO: Find a way to not use magic numbers
		let user: UserCred = UserCred {
			name: inpt[0..64].to_vec(),
			access: inpt[64],
			salt: inpt[65..81].to_vec(),
			pash: inpt[81..145].to_vec(),
		};
		users.push(user);
		i += 145;
	}
	users
}

// TODO: Seems unintuitive that adding a user currently requires 2 actions (push to key, update key); refactor?
pub fn update_key(update: Vec<UserCred>) -> Result<(), &'static str> {
	let key = File::open(KEY_FILE).expect("Unable to open key file!");
	//let mut reader = BufReader::new(key);
	let mut writer = BufWriter::new(key);
	for user in update {
		write!(writer, "{}", user);
	}
	writer.flush().expect("I'm a dumb idiot!");
	Ok(())
}

impl UserCred {
	pub fn new() -> Result<UserCred, Box<dyn Error>> {
		// TODO NEW WOO WOO

		// Standard input stream, Hasher config, temporary variables
		let inpt = io::stdin();
		let mut out = io::stdout();
		let config = argon2::Config::default();
		let mut tmp1 = String::new();
		let mut tmp2 = String::new();
		let mut okay: bool = false;
		let mut new_user: UserCred = UserCred{
			name: Vec::with_capacity(LARGE_BLOCK_SIZE),
			access: 0,
			salt: Vec::with_capacity(SMALL_BLOCK_SIZE),
			pash: Vec::with_capacity(LARGE_BLOCK_SIZE),
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
				if c.is_ascii_whitespace() || tmp1.len() > LARGE_BLOCK_SIZE { okay = false; }
			}
			if !okay { 
				println!("User names may not contain spaces, tabs, or newlines.");
				tmp1.clear();
			}
		}
		for c in tmp1.as_bytes() {
			new_user.name.push(*c);
		}
		while new_user.name.len() < new_user.name.capacity() {
			new_user.name.push(9u8);
		}
		okay = false;
		tmp1.clear();

		// Set read/write access
		let mut n: u8 = 0;
		okay = false;
		while !okay {
			okay = true;
			println!("Set user access level:");
			println!("    1) Read-Only");
			println!("    2) Read & Edit");
			print!(" > ");
			out.flush()?;
			inpt.read_line(&mut tmp1)?;
			tmp1 = String::from(tmp1.trim());
			n = tmp1.parse().expect("Valid access levels are 1 and 2.");
			if (n < 1) || (n > 2) { okay = false; }
			if !okay {
				println!("Valid access levels are 1 and 2.");
				tmp1.clear();
			}
		}
		new_user.access = n;
		tmp1.clear();
		
		// Get a password to use; validate by double-checking
		okay = false;
		println!("Please choose a password.  Passwords are case-sensitive.");
		println!("For security, the password you type will not appear on screen;");
		println!("You will be asked to type it again to confirm.");
		while !okay {
			okay = true;
			tmp1 = rpassword::prompt_password_stdout("Password: ").unwrap().to_string();
			tmp2 = rpassword::prompt_password_stdout("Re-enter Password: ").unwrap().to_string();
			if tmp1 != tmp2 || tmp1.len() > LARGE_BLOCK_SIZE { okay = false; }
			if !okay {
				println!("Your first and second entries did not match.  Try again.");
				tmp1.clear();
				tmp2.clear();
			}
		}
		println!("Please remember your password, as it cannot be recovered.");
		
		// Get a salt value;  flushing out the first value produced by OsRng
		// is recommended when using it early in the boot sequence; obviously
		// that isn't the case here, but it should have been, so I'm including it.
		let mut peanuts = [0u8; SMALL_BLOCK_SIZE];
		OsRng.fill_bytes(&mut peanuts);
		peanuts = [0u8; SMALL_BLOCK_SIZE];
		OsRng.fill_bytes(&mut peanuts);
		
		//new_user.salt = peanuts;
		for b in &peanuts {
			new_user.salt.push(*b);
		}
		while new_user.salt.len() < new_user.salt.capacity() {
			new_user.salt.push(9u8);
		}
		
		// Hash the salted password string using the Argon2 algorithm
		let hash = argon2::hash_encoded(tmp1.as_bytes(), &peanuts, &config)?;
		for b in hash.split_at(28).1.as_bytes() { // 28 is the beginning of the actual hash value
			new_user.pash.push(*b);
		}
		while new_user.pash.len() < new_user.pash.capacity() {
			new_user.pash.push(9u8);
		}
		Ok(new_user)
	}

	pub fn equals(&self, other: &UserCred) -> bool {
		if self.name == other.name &&
		   self.access == other.access &&
		   self.salt == other.salt && 
		   self.pash == other.pash {
			true
		} else {
			false
		}
	}
	
	pub fn verify(&self, key: Vec<UserCred>) -> Result<bool, &str> {
		let mut verf: bool = false;
		for other in key {
			if self.equals(&other) {
				verf = true;
			}
		}
		match verf {
			true => Ok(self.access >= 2),
			false => Err("Unable to verify user credentials.")
		}
	}

	pub fn name(&self) -> String {
		String::from_utf8(self.name.clone()).unwrap()
	}
	pub fn access(&self) -> u8 {
		self.access
	}
	pub fn salt(&self) -> &Vec<u8> {
		&self.salt
	}
	pub fn pash(&self) -> &Vec<u8> {
		&self.pash
	}
}