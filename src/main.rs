//Declare dependencies
use std::io::stdin;

fn main() {
	//Declare a mutable input string
    let mut input_string = String::new();
    println!("Enter your text here: ");
    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");
    println!("Your input is {}", input_string)
}