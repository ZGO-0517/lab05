// implement input library
use std::io;


// main script
fn main() {
	// gathers start of range input from user
	println!("Start of range:");	
	let mut start_input = String::new();
	
	// fetches the text and handles potential errors
	io::stdin()
		.read_line(&mut start_input).expect("Failed to read line");
	// typecasts the input as an integer
	let start: i32 = start_input
		.trim().parse().expect("Please type a valid number");
	// calls the functions below

	// repeat for the end of the range
	println!("End of range:");
	let mut end_input = String::new();
	io::stdin()
		.read_line(&mut end_input).expect("Failed to read line");
	let end: i32 = end_input
		.trim().parse().expect("Please type a valid number");

	// loops the results for the range
	for n in start..=end {	
		println!(
			"{n}: even={}, digit_sum={}, prime={}, divisors={}",
            		is_even(n), digit_sum(n), is_prime(n), count_divisors(n)
		);		
	}
}


// checking if even
fn is_even(n: i32) -> bool {
	// this equation checks if the input is even and returns a boolean value
	n % 2 == 0
}


// calculating digit sum
fn digit_sum(n: i32) -> i32{
	// create a new variable that will be the typecast of n
	let mut num = n.abs() as u32;
	// create a sum variable
	let mut sum = 0;
	
	// start a loop that will add the individual digits one by one until
	// the typecasted n (num) becomes 0
	while num > 0 {
		sum += num % 10;
		num /= 10;
	}
	// returns the digit sum of the input
	sum as i32
}


// checks if input is a prime number
fn is_prime(n: i32) -> bool {
	// checks if the number is 1 or below (all not prime numbers)	
	if n <= 1 { return false; }

	// checks if the number is 2 (the only even prime number)
	if n == 2 { return true; }

	// checks if the number is even (all not prime except for 2)
	if n % 2 == 0 { return false; }

	// checks for odd divisors to the square root of n
	let limit = (n as f64).sqrt() as i32;
	let mut prime = true;

	// checks to see if smaller integers can divide it evenly
	for i in (3..=limit).step_by(2) {
		if n % i == 0 {
			prime = false;
			break;
		}
	}
	// returns a bool value for the loop
	prime
}


// checks how many integers from 1 to n divide n evenly
fn count_divisors(n: i32) -> i32 {
	// if a number is 0 or less it will have no divisors
	if n <= 0 { return 0; }

	// creates a variable for counting the divisors
	let mut divisors = 0;
	for i in 1..=n {
		// checks if the number divides n evenly
		if n % i == 0 {
			divisors += 1;
		}
	}
	// returns the amount of divisors
	divisors
}
