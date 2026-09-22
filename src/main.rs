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
	digit_sum(n);
}

//checking if even
fn is_even(n: i32) {
	//create a boolean that comes back true if the remainder is 0
	let even = n % 2 == 0;

	//print the result of either an even or odd number
	if even {
		println!("The number is even!");
	}
	else {
		println!("The number is odd!");
	}
}

//calculating digit sum
fn digit_sum(n: i32) {
	// create a new variable that will be the typecast of n
	let mut num = n.abs() as u32;
	//create a sum variable
	let mut sum = 0;
	
	// start a loop that will add the individual digits one by one until
	// the typecasted n (num) becomes 0
	while num > 0 {
		sum += num % 10;
		num /= 10;
	}
	//print the digit sum
	println!("The digit sum is {}", sum);
}
