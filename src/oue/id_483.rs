use std::io;

/**
 * Read a number through the input buffer and print "par" if the number are pair
 * or "impar" if the number ar impair.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = String::new();

	// Read user input from the console and handle possible errors.
	stdin.read_line(&mut input).expect("Failed to read line");

	let x: i32 = input
		.trim() //Removes any whitespace at the beginning or end of the string.
		.parse()// Convert input into a number.
		.expect("Cannot parse number"); // Handle possible errors

	// Print "par" if the number are pair or "impar" if the number ar impair.
	if x % 2 == 0 {
		print!( "par");
	}
	else {
		print!( "impar");
	}
}