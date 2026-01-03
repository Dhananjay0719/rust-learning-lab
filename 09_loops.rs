fn main() {
    for i in 1..=5 {
        println!("i = {}", i);
    }

    let arr = [10, 20, 30, 40];

    for value in arr.iter() {
        println!("{}", value);
    }

    for (index, value) in arr.iter().enumerate() {
        println!("{} -> {}", index, value);
    }
}
