use std::io; // Library for inputs
use rand::Rng; // Library for random
use std::cmp::Ordering; // Library for ordering


fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    println!("Please type your guess: \n");
    
    let mut guess = String::new(); // Mutable variable
    
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");


    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You win"),
    }


    
}
