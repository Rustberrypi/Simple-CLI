extern crate argon2;
extern crate rpassword;
use std::boxed::Box;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use rand::RngCore;
use rand::rngs::OsRng;

const ARGON: &str = "$argon2i$v=19$m=4096,t=3,p=1"; // 28 bytes
const KEY_FILE: &str = "./.nothing.key";
const LARGE_BLOCK_SIZE: usize = 128;
const SMALL_BLOCK_SIZE: usize=16;
const JUNK_BYTE: u8 = 33u8;

pub struct UserCred {
	name: Vec<u8>,
	access: u8,
	//salt: Vec<u8>,
	pash: Vec<u8>,
}

pub struct CredKey {
	raw: Vec<u8>,
	users: Vec<UserCred>,
	file_path: String,
	secret: u8,
}

// TODO: Create functions to translate this enum to & from single bytes
pub enum AccessLevel {
	ReadOnly,
	ReadWrite,
}

impl fmt::Display for UserCred {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}{}",
				String::from_utf8_lossy(&self.name),
				self.access.to_string(),
				//String::from_utf8_lossy(&self.salt),
				String::from_utf8_lossy(&self.pash),
			)
	}
}

impl PartialEq for UserCred {
	fn eq(&self, other: &Self) -> bool {
		self.name.eq(&other.name) &&
		self.access.eq(&other.access) &&
		//self.salt.eq(&other.salt) && 
		self.pash.eq(&other.pash)
	}
}
impl Eq for UserCred { }

pub fn turn_key() -> Result<CredKey, &'static str> {
	let mut keylib: CredKey;
	keylib = CredKey {
		raw: Vec::new(),
		users: Vec::new(),
		file_path: String::from(KEY_FILE),
		secret: 0,
	};
	keylib.load()?;
	keylib.parse()?;
	Ok(keylib)
}

fn pack_vector(val: Vec<u8>, cap: usize) -> Vec<u8> {
	let mut ret: Vec<u8> = Vec::with_capacity(cap);
	for byte in val {
		ret.push(byte);
	}
	while ret.len() < ret.capacity() {
		ret.push(JUNK_BYTE);
	}
	ret
}

impl UserCred {
	pub fn new() -> Result<UserCred, Box<dyn Error>> {
		// Standard input stream, Hasher config, temporary variables
		let inpt = io::stdin();
		let mut out = io::stdout();
		let config = argon2::Config::default();
		let new_name: Vec<u8>;
		let new_access: u8;
		let new_salt: Vec<u8>;
		let new_pash: Vec<u8>;
		let mut tmp1 = String::new();
		let mut tmp2 = String::new();
		let mut okay: bool = false;
		let new_user: UserCred;
		// let mut new_user: UserCred = UserCred {
		// 	name: Vec::with_capacity(LARGE_BLOCK_SIZE),
		// 	access: 0,
		// 	salt: Vec::with_capacity(SMALL_BLOCK_SIZE),
		// 	pash: Vec::with_capacity(LARGE_BLOCK_SIZE),
		// };

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
		new_name = pack_vector(tmp1.as_bytes().to_vec(), LARGE_BLOCK_SIZE);
		// for c in tmp1.as_bytes() {
		// 	new_user.name.push(*c);
		// }
		// while new_user.name.len() < new_user.name.capacity() {
		// 	new_user.name.push(JUNK_BYTE);
		// }
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
		// new_user.access = n;
		new_access = n;
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
		// for b in &peanuts {
		// 	new_user.salt.push(*b);
		// }
		// while new_user.salt.len() < new_user.salt.capacity() {
		// 	new_user.salt.push(JUNK_BYTE);
		// }
		new_salt = peanuts.to_vec();
		
		// Hash the salted password string using the Argon2 algorithm
		// let hash = argon2::hash_encoded(tmp1.as_bytes(), &peanuts, &config)?;
		let hash = argon2::hash_encoded(tmp1.as_bytes(), &new_salt, &config)?;
		// println!("UserCred::new() has calculated the following hash:\n{}", hash);
		// for b in hash.split_at(28).1.as_bytes() { // 28 is the beginning of the actual hash value
		// 	new_user.pash.push(*b);
		// }
		// while new_user.pash.len() < new_user.pash.capacity() {
		// 	new_user.pash.push(JUNK_BYTE);
		// }
		new_pash = pack_vector(hash.split_at(28).1.as_bytes().to_vec(), LARGE_BLOCK_SIZE);
		// println!("UserCred::new() is creating:\n{}", new_user);
		new_user = UserCred {
			name: new_name,
			access: new_access,
			//salt: new_salt,
			pash: new_pash,
		};
		Ok(new_user)
	}

	// Create a new user from provided data; no stdio involvement
	fn new_from(nvec: Vec<u8>, pvec: Vec<u8>, a: u8) -> Result<UserCred, &'static str> {
		let config = argon2::Config::default();
		let mut peanuts = [0u8; SMALL_BLOCK_SIZE];
		OsRng.fill_bytes(&mut peanuts);
		peanuts = [0u8; SMALL_BLOCK_SIZE];
		OsRng.fill_bytes(&mut peanuts);

		let hash = argon2::hash_encoded(&pvec, &peanuts, &config).expect("Bad hash, bruh.");
		
		let new_user = UserCred {
			name: pack_vector(nvec, LARGE_BLOCK_SIZE),
			access: a,
			//salt: peanuts.to_vec(),
			pash: pack_vector(hash.split_at(28).1.as_bytes().to_vec(), LARGE_BLOCK_SIZE),
		};
		Ok(new_user)
	}

	pub fn verify(&self, key: Vec<UserCred>) -> Result<bool, &str> {
		let mut verf: bool = false;
		for other in key {
			if self.eq(&other) {
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
	// pub fn salt(&self) -> &Vec<u8> {
	// 	&self.salt
	// }
	pub fn pash(&self) -> &Vec<u8> {
		&self.pash
	}
}

impl CredKey {
	pub fn new(file: String) -> Result<CredKey, &'static str> {
		File::create(Path::new(&file)).expect("Failed to create new key file.");
		let new_key = CredKey {
			raw: Vec::new(),
			users: Vec::new(),
			file_path: file,
			secret: 0,
		};
		Ok(new_key)
	}

	pub fn load(&mut self) -> Result<(), &'static str> {
		let mut f = OpenOptions::new()
					.read(true)
					.open(&self.file_path)
					.expect("Error reading specified key file.");
		f.read_to_end(&mut self.raw).expect("Error reading key file.");
		// println!("{}", String::from_utf8_lossy(&self.raw));
		// match self.raw.len() > 0 {
		// 	true => Ok(()),
		// 	false => Err("No entries found in key file!".into()),
		// }
		Ok(())
	}

	fn parse(&mut self) -> Result<(), &'static str> {
		if self.raw.len() > 144 {
			self.users = Vec::new();
			// let mut users: Vec<UserCred> = vec!();
			let mut i: usize = 0;		
			while i < self.raw.len() {
				// println!("Starting from {}...", i);
				// TODO: Find a way to not use magic numbers
				let user: UserCred = UserCred {
					// name: self.raw[0..64].to_vec(),
					name: self.raw[i..i + LARGE_BLOCK_SIZE].to_vec(),
					// access: self.raw[64],
					access: self.raw[i + LARGE_BLOCK_SIZE],
					// salt: self.raw[65..81].to_vec(),
					//salt: self.raw[LARGE_BLOCK_SIZE+1..LARGE_BLOCK_SIZE+1+SMALL_BLOCK_SIZE].to_vec(),
					// pash: self.raw[81..145].to_vec(),
					pash: self.raw[(i + LARGE_BLOCK_SIZE)+1..i + (2*LARGE_BLOCK_SIZE)+1].to_vec(),
				};
				let newname: Vec<u8> = self.raw[i..i+LARGE_BLOCK_SIZE].to_vec();
				// println!("Parsed user {}", String::from_utf8_lossy(&newname));
				// println!("{}", &user);
				self.users.push(user);
				i += (2*LARGE_BLOCK_SIZE)+1;
			}
		}
		if self.users.len() < 1 {
			println!("No users in this key.  Let's create one.");
			let nuser = UserCred::new().expect("UH OH SPAGHETTIOS");
			self.append(nuser)?;
			self.save()?;
		}
		match self.users.len() > 0 {
			true => Ok(()),
			false => Err("Nobody is anybody.".into()),
		}
	}

	pub fn save(&mut self) -> Result<(), &'static str> {
		let mut data: Vec<u8> = Vec::new();
		for u in 0..self.users.len() {
			data.append(&mut self.users[u].name.clone());
			data.push(self.users[u].access.clone());
			//data.append(&mut self.users[u].salt.clone());
			data.append(&mut self.users[u].pash.clone());
		}
		let mut f = OpenOptions::new()
					.write(true)
					.open(&self.file_path)
					.expect("Unable to write to key file.");
		f.write_all(&data[..]).expect("Unexpected failure writing key file.");
		Ok(())
	}

	pub fn add_user(&mut self, name: String, pass: String, acc: u8) -> Result<(), &'static str> {
		let name_bytes = name.as_bytes().to_vec();
		let pass_bytes = pass.as_bytes().to_vec();
		let mut dupe: bool = false;
		for u in &self.users {
			if name_bytes.eq(&u.name) {
				dupe = true;
			}
		}
		if !dupe {
			self.users.push(UserCred::new_from(name_bytes, pass_bytes, acc)?);
		}
		match dupe {
			false => Ok(()),
			true => Err("That user name is not available."),
		}
	}


	pub fn append(&mut self, new: UserCred) -> Result<(), &'static str> {
		let num_before = self.users.len();
		self.users.push(new);
		let num_after = self.users.len();
		match num_after > num_before {
			true => Ok(()),
			false => Err("Unexpected error adding user."),
		}
	}
	
	pub fn verify(&self, name: &str, password: &str) -> Option<&UserCred> {
		let verified: &UserCred;
		let name_bytes = pack_vector(name.as_bytes().to_vec(), LARGE_BLOCK_SIZE);
		let pass_bytes = password.as_bytes();
		// println!("Attempting to verify user credentials ({}, {})...", String::from_utf8_lossy(&name_bytes), String::from_utf8_lossy(&pass_bytes));
		for u in &self.users {
			// rintln!("Comparing against user {}~~~", u.name());
			if u.name.eq(&name_bytes) {
				// println!("YES THEY ARE THE SAMEULAR");
				// println!("{}", String::from_utf8_lossy(&u.pash).trim_end_matches("!").to_string());
				let arg_hash: String = String::from(ARGON) + &String::from_utf8_lossy(&u.pash).trim_end_matches("!").to_string();
				// println!("{}", arg_hash);
				//arg_hash.append(&mut u.pash.clone());
				if argon2::verify_encoded(&arg_hash, pass_bytes).expect("Error in Argon2 password verification") {
					verified = u;
					println!("User credentials verified.");
					return Some(verified)
				}
			} else {
				// println!("NO HE NOT A SAME FROM {}", String::from_utf8_lossy(&u.name));
			}
		}
		None
	}

	pub fn raw(&self) -> &Vec<u8>{
		&self.raw
	}
	pub fn users(&self) -> &Vec<UserCred> {
		&self.users
	}
	pub fn file_path(&self) -> &String {
		&self.file_path
	}
	
}