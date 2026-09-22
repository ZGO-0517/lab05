// implement input library
use std::io;


// main script
fn main() {
	// gathers input from user
	println!("Please enter a number:");	
	let mut n = String::new();
	
	// fetches the text and handles potential errors
	io::stdin()
		.read_line(&mut n).expect("Failed to read line");
	// typecasts the input as an integer
	let n: i32 = n
		.trim().parse().expect("Please type a valid number");
	// calls the functions below
	println!("");
	is_even(n);
	digit_sum(n);
	is_prime(n);
	count_divisors(n);
}


// checking if even
fn is_even(n: i32) {
	// create a boolean that comes back true if the remainder is 0
	let even = n % 2 == 0;

	// print the result of either an even or odd number
	if even {
		println!("{} is even!", n);
	}
	else {
		println!("{} is odd!", n);
	}
}


// calculating digit sum
fn digit_sum(n: i32) {
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
	// print the digit sum
	println!("The digit sum is {}!", sum);
}


// checks if input is a prime number
fn is_prime(n: i32) {
	// checks if the number is 1 or below (all not prime numbers)	
	if n <= 1 {
		println!("{} is not a prime number!", n);
		return;
	}
	// checks if the number is 2 (the only even prime number)
	if n == 2 {
		println!("{} is a prime number!", n);
		return;
	}
	// checks if the number is even (all not prime except for 2)
	if n % 2 == 0 {
		println!("{} is not a prime number!", n);
		return;
	}
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
	// prints the results
	if prime {
		println!("{} is a prime number!", n);
	}
	else {
		println!("{} is not a prime number!", n);
	}
}


// checks how many integers from 1 to n divide n evenly
fn count_divisors(n: i32) {
	// if a number is 0 or less it will have no divisors
	if n <= 0 {
		println!("{} has no divisors!", n);
		return;
	}
	// creates a variable for counting the divisors
	let mut divisors = 0;
	for i in 1..=n {
		// checks if the number divides n evenly
		if n % i == 0 {
			divisors += 1;
		}
	}
	// prints the result
	println!("{} has {} divisor(s).", n, divisors);
}
