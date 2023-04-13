use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod main_second;

fn main(){
    main_second::second_function();
    // guessing_game();
}

fn guessing_game() {
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
    
    println!("You guessed: {guess}");
    let mut counter = 0;
    let mut prev_number = 0;
    let mut ret_number: u32 = 0;
    let mut prev_state = 0;
    let mut minvalue: u32 = 0;
    let mut maxvalue: u32 = 100;

    loop {

    // check
    if counter == 10 {
      break;
    }
    
    if counter > 0 {
        ret_number = binary_search(prev_number,minvalue,maxvalue,state);
    }else {
        ret_number = guess;
    }
    println!("{}.Begin new line. prevnumber : {}, ret_number: {}, minnumber : {}, maxnumber : {}",counter+1,prev_number,ret_number,minvalue,maxvalue);

    match ret_number.cmp(&secret_number) {
        Ordering::Less => {
            println!("{} is Too small!",ret_number);
            if state == 1 {
                // prev value is still too small
                minvalue = minvalue; 
            }else {
                minvalue = ret_number;
            }
            state =  1;
            counter+= 1;
            prev_number = ret_number;
            
        }
        Ordering::Greater => {
            println!("{} is Too big!",ret_number);
            
            if state == 2 {
                // prev value is still too big
                maxvalue=maxvalue;
            }else {
                maxvalue = ret_number;
            }
            prev_number = ret_number;

            state =  2;
            counter+=1;
        }
        Ordering::Equal => {
            println!("{} is correct, You win!",ret_number);
            //break;
        }
    }
    }
}

fn binary_search(num: u32,minvalue: u32,maxvalue: u32, state: u32) -> u32 {

    //let mut result: u32;

    match state {
        1 => {
            // value is too small, add more number
            if num % 2 == 1 {
                maxvalue-((num+1)/2)
            }else {
                maxvalue-(num/2)
            }
        },
        2 => {
            // value is too big, substract number
           if (num-minvalue) % 2 == 1 {
                minvalue + ((num-minvalue)+1/2)
            }else {
                minvalue + ((num-minvalue)/2)
            }
        },
        _ => {
            0
        }
    }
}
