//Shadowing in Rust Language

fn main(){
    let x=5;
    println!("x={}",x);
    let x="Hello World";
    println!("x={}",x);
    let x=x.len();
    println!("x={}",x);
}
