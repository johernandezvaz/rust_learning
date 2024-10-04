use std::io; // Library for inputs
use rand::Rng; // Library for random
use std::cmp::Ordering; // Library for ordering


fn main() {
    println!("Guess the number!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please type your guess: ");
        
        let mut guess = String::new(); // Mutable variable
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        
        let guess: u32 =  match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => continue,
        };
        
            
    
        
    
        println!("You guessed: {guess}");
        println!("The secret number is: {secret_number}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
        }
    }
}
}