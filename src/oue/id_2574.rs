// Import BufRead from std::io to handle input buffer
use std::io::{self, BufRead};

/**
 * Read 3 numbers through the input buffer, reverse the order and print each number separated by 'enter'.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = stdin.lock();

	// Initialize variables.
	let mut n1 = String::new();
	let mut n2 = String::new();
	let mut n3 = String::new();

	// Read each number separated by 'enter'.
	input.read_line(&mut n1).expect("Failed to read line");
	input.read_line(&mut n2).expect("Failed to read line");
	input.read_line(&mut n3).expect("Failed to read line");

	// Print each number separated by 'enter'.
	print!("{}{}{}", n3, n2, n1);
}