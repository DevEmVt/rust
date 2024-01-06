use std::io;

/**
 * Read 2 numbers through the input buffer separated by a space,
 * print the multiplication of both numbers.
**/
pub fn main() {
	// Initialize standard input.
	let stdin = io::stdin();
	let mut input = String::new();

	// Read user input from the console and handle possible errors.
	stdin.read_line(&mut input).expect("Failed to read line");

	// Removes any whitespace at the beginning or end of the string.
	let input = input.trim();

	/*
	 * Splits the string into words separated by whitespace,
	 * then converts each word to an integer and puts them into a vector
	 */
	let numbers: Vec<i32> = input
		.split_whitespace() // Split the string into words.
		.map( |n| n.parse().expect("Cannot parse number")) // Convert each word into a number.
		.collect(); // Put the numbers into a vector.

	// Verify that exactly two numbers have been entered
	if numbers.len() != 2 {
		println!("Please enter two numbers separated by a space.");
		return; // Exits the program if two numbers are not entered.
	}

	// Print the result of the multiplication.
	print!( "{}", numbers[0] * numbers[1] );
}