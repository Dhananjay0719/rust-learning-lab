fn main(){
    let str1=String::from("Dhananjay"); // The Heap allocated types like String do not implement the Copy trait so ownership will be moved instead of copy
    let str2=str1; //Ownership Transfer......str2 is the new owner 

    /*
     println("{}",str1); throws error now
     */
   
    println!("The new owner is str2 which points to {}",str2);
}


//Two Owners were prevented by Rust because third rule of ownership says that when vars go out of scope they will free but due to this Double Free Error would have occured
