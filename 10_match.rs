fn main() {
    let day = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid"),
    }

    let grade = 'A';

    let result = match grade {
        'A' => "Excellent",
        'B' => "Good",
        'C' => "Average",
        'D' => "Poor",
        _ => "Fail",
    };

    println!("{}", result);
}
