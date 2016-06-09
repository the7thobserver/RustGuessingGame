extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

	// 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

		// Variables are immutable by default
		// 'mut' makes this variables mutable
		// :: signifies it's an 'associated function' aka static method
        let mut guess = String::new();

		// Use the io library 
		// & is a reference to the variable 
		// expect(...) is like throwing an exception, the argument will be displayed with a panic!
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

		// Guess is now a unsigned 32 bit integer, immutable 
		// Use match to determine how to handle the error
        let guess: u32 = match guess.trim().parse() {
			// What's a valid, sets num (on the right) to the Ok value (integer)
            Ok(num) => num,
			// _ is catch all
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
			// Ordering is an enum w/ Less, Greater, Equal
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}