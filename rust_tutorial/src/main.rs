#![allow(unused)]

//using the standard IO library for rust
use std::io;

fn main() {
	println!("Guess the number!!");
	println!("Please input your number");

	// in rust, we add mut to the variable name
	let mut guess = String::new();

	// this is kind of like try catch concept
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed--> {guess}");
}
