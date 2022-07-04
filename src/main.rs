use std::io;
use std::io::Write;

fn main() {
	loop {
		let mut input: String = String::new();
		print!("> ");
		std::io::stdout().flush().unwrap();

		match io::stdin().read_line(&mut input) {
			Ok(_) => println!("Good"),
			Err(_) => println!("Bad"),
		}
	}
}
