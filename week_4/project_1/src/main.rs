// A rust program that tells how fast a car travels in kilometers

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("Enter first distance of the car: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let d1:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter first time of the car: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t1:f32 = input2.trim().parse().expect("Not a valid number");

    let speed1:f32 = (d1 * 1.609344) / t1;
    println!("First speed of the car: {}", speed1);

    println!("Enter the second distance of the car: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let d2:f32 = input3.trim().parse().expect("Not a valid number");

    println!("Enter second time of the car: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let t2:f32 = input4.trim().parse().expect("Not a valid number");

    let speed2:f32 = (d2 * 1.609344) / t2;
    println!("Second speed of the car: {}", speed2);


     



    
}
