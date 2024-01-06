// Import BufRead from std::io to handle input buffer
use std::io::{self, BufRead};

/**
 * Read a number through the input buffer, print the number multiply by 60.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = stdin.lock();

	// Initialize variables.
	let mut n = String::new();

	// Read a number.
	input.read_line(&mut n).expect("Failed to read line");

	// Parse input to an unsigned 32-bit integer (u32)
	let n: u32 = match n.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};

	// Print the input multiply by 60
	print!( "{}", n*60 );
}