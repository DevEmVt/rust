use std::io;

/**
 * Read n numbers through the input buffer separated by a space until reading 0,
 * print the sum of the numbers.
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
		.take_while(|&x| x != 0) // Take numbers until encountering 0.
		.collect(); // Put the numbers into a vector.

	// Calculate sum of all the numbers.
	let sum: i32 = numbers.iter().sum();

	// Print the result of the multiplication.
	print!( "{}", sum );
}