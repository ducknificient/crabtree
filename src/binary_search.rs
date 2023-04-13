use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn entry(){
    
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    let mut state: u32 = 0;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let secret_number = 41;
    let guess = 58;

    let mut low: u32 = 0;
    let mut high: u32 = 100;

    while low <= high {
      int mid = low + (high - low) / 2;

      if (secret_number == guess)
        return guess;

      if (secret_number < x)
        low = mid + 1;

      else
        high = mid - 1;
    }

}

fn binary_search(){

}
