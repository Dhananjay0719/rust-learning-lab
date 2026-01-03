fn main(){
    // let mut vect : Vec<i32>= Vec::new();

    let mut vect=Vec::<i32>::new();

    vect.push(1);
    vect.push(2);
    vect.push(3);

    println!("The Vector is {:?}",vect);

    let mut vect2=vec![1,2,3,4];
    vect2.push(5);
    println!("The Vector is {:?}",vect2);
}
