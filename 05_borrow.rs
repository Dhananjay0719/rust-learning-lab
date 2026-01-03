fn main(){
    let mut str1=String::from("Hello Jiii");
    // let len=stringlen(&str1); //Borrowing the value of str1
    // println!("The length of {} is {}",str1,len);

    stringwrite(&mut str1);
}

fn stringlen(str2: &str)->usize { 
    //Imp Note: if this fn borrow using &String stringlen("Hello"); wont work &str allows both String & str literals
    return str2.len();
    /* Rust uses Non-Lexical Lifetimes (NLL), meaning a borrow ends at its LAST USE,
    not at the end of the scope. If an immutable borrow is no longer used,
    Rust allows a mutable borrow afterward. This ensures memory safety while
    remaining flexible and preventing read-write or write-write conflicts. */
}

fn stringwrite(str3 : &mut String) {
    str3.push_str(" World");
}
