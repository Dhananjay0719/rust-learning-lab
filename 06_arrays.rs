fn main(){
    let arr1=[1,2,3];
    write_arr(arr1);
    println!("The arr inside main is {:?}",arr1);
}

fn write_arr(mut arr2: [u8;3]){
    arr2[1]=5;
    println!("The arr inside func is {:?}",arr2);
}

fn len_arr(arr3:[u8;3]) -> usize{
    return arr3.len();
}
