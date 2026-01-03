use std::io;

fn main(){
    let mut input=String::new();
    println!("Enter Your Input:");
    io::stdin().read_line(&mut input).expect("User Input Failed");
    println!("User Input is {}",input);
}

