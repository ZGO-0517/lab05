//implement input library
use std::io;

//main script
fn main() {
	println!("Please enter a number:");	
	let mut n = String::new();
	
	io::stdin()
		.read_line(&mut n).expect("Failed to read line");

	let n: i32 = n
		.trim().parse().expect("Please type a valid number");

	is_even(n);
}

//checking if even
fn is_even(n: i32) {
	let even = n % 2 == 0;

	if even {
		println!("The number is even!");
	}
	else {
		println!("The number is odd!");
	}
}
