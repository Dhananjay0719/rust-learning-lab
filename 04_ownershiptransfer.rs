fn main(){
    str1=String::from("Hello"); //str1 is owner
    print_str(str1); //Ownership transfer
    println!("The Value of String inside main is {}",str1); //Throws Error

}

fn print_str(str2:String){
    println!("The Value of String inside print_str is {}",str2);
    //after this value will be dropped
}
