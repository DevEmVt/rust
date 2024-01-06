// Import BufRead from std::io to handle input buffer
use std::io::{self, BufRead};

/**
 * Read 3 numbers through input buffer, use the numbers to use some arithmetic operations.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = stdin.lock();

	// Initialize variables.
	let mut c = String::new();
	let mut p = String::new();
    let mut h = String::new();

	// Read each number separated by 'enter'.
	input.read_line(&mut c).expect("Failed to read line");
	input.read_line(&mut p).expect("Failed to read line");
    input.read_line(&mut h).expect("Failed to read line");

	// Parse input to an unsigned 32-bit integer (u32)
	let mut c: u32 = match c.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};

	let p: u32 = match p.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};

	let mut h: u32 = match h.trim().parse() {
		Ok(num) => num,
		Err(_) => {
			println!("Please enter a valid number.");
			return;
		}
	};

	// Some cool operations.
	h += 1;
	c -= p;

	// Print the result.
	print!( "{}", p + ( c % h ) );
}