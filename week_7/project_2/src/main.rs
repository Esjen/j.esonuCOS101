use std::io;

fn main() {
   loop{
    let mut input1 = String::new();
    println!("\nEnter the Number of Siblings that you have: {}", input1);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1:f32 = input1.trim().parse().expect("Not a valid number");
    let input1_arr:[i32,7] = [0,1,2,3,4,5,6];
    for index in 0..7 {
        println!("Number of Siblings is :{}", index,input1_arr[index]);

    let mut input2 = String::new();
    println!("\nEnter the First name of each Sibling: [{}]", input2);
    io::stdin(),read_line(&mut input2).expect("Not a valid string");
    let input2 = input2.trim();

    let mut input3 = String::new();
    println!("\nEnter the Age of the Number of Siblings that you have: [{}]", input3);
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input3:f32 = input3.trim().parse().expect("Not a valid number");






}
