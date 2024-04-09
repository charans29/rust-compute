use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Guess the Secret number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Please input your Guess.");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    println!("and...... ");

    thread::sleep(Duration::from_secs(3));

    println!("The secret number is: {secret_number}");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("YOUR GUESS is Too small!"),
        Ordering::Greater => println!("YOUR GUESS is Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
