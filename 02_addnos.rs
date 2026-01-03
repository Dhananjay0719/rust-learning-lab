fn main(){
    let num1 : u8=20;
    let num2 : u8=30;
    let ans=add(num1,num2);
    println!("{}",ans);
}
 
//function for adding two numbers
fn add(num1:u8,num2:u8)-> u8{
    return num1+num2;
}
