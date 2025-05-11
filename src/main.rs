use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   
    println!("Hello Welcome to my guessing game!!!");
    
    let secret_number = rand::rng()
								.random_range(1..=10);
    
    println!("Guess a number between 1 to 10, if we are guessing the same numbers then we are best friends");

    let mut guess = String::new();
    
    io::stdin()
		.read_line(&mut guess)
		.expect("Error reading the line");
    
    let guess: u32 = match guess
		.trim()
		.parse(){
			Ok(num) => num,
			Err(_) => 0
		};
    
    println!("You guessed: {}", guess);
    println!("I guessed: {}", secret_number);
    
    match guess.cmp(&secret_number){
	Ordering::Less =>{
			println!("Too Less!!");
			main();},
	Ordering::Greater => {
			println!("Too Greater");
			main();},
	Ordering::Equal => println!("We are close Friends!!")
	}

    
}


 
