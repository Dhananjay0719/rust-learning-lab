use std::io;
use rand::Rng;

fn main(){
    let guesses:[&str;4]=["Apple","Banana","Orange","Grapes"];
    let random_index = rand::thread_rng().gen_range(0..guesses.len());
    let secret = guesses[random_index];
    println!("Select one of the following:");
    for fruits in guesses{
        println!("{}",fruits);
    }

    loop{
    let mut input=String::new();
    println!("Enter your input:");
    io::stdin().read_line(&mut input).expect("User Input Failed");

    let guess=input.trim();
    if guess == secret {
        println!(" Correct! The fruit was {}", secret);
        break;
    } else {
        println!("Retry");
    }
    }
}
