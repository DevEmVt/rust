// Import BufRead from std::io to handle input buffer
use std::io::{self, BufRead};

/**
 * Read 2 numbers through input buffer, print multiplication of both numbers.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = stdin.lock();

	// Initialize variables.
	let mut a = String::new();
	let mut b = String::new();

	// Read each number separated by 'enter'.
	input.read_line(&mut a).expect("Failed to read line");
	input.read_line(&mut b).expect("Failed to read line");

	// Parse input to an unsigned 32-bit integer (u32)
	let a: u32 = match a.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};

	let b: u32 = match b.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};
	// Print each number separated by 'enter'.
	print!( "{}", a*b );
}