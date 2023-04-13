use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}");


    println!("Please input your guess.");

    let mut guess = String::new();
    let mut state = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    
    println!("You guessed: {guess}");

    loop {

    let guess = binary_search(guess,&state);
    println!("Number is : {guess} now");

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!");
            state =  "up".to_string();
        }
        Ordering::Greater => {
            println!("Too big!");
            state =  "down".to_string();
        }
        Ordering::Equal => {
            println!("You win!");
            //break;
        }
    }
    }
}

fn binary_search(num: u32, state: &str) -> u32{

    let mut result: u32 = 0;

    match state {
        "up" => {
            if num % 2 == 1 {
                result = (((num-50)+1)/2)+50;
            }else {
                result =((num-50)/2)+50;
            }
        },
        "down" => {
           if num % 2 == 1 {
                result = (num+1)/2;
            }else {
                result = num/2;
            }
        },
        _ => {
            result = 0;
        }
    }
    result
}
