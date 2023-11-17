//Rust program to display the multiplication table from 1 to n

use std::io;

fn main() {
   loop {

    let mut input1 = String::new();
    println!("Enter the Lower Bound: {}", input1);
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to input");

    let mut input2 = String::new();
    println!("Enter the Upper Bound: {}", input2);
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let upper_bound:i32 = input2.trim().parse().expect("Failed to input");

    let mut value = String::new();
    println!("Enter Any Value: {}", value);
    io::stdin().read_line(&mut value).expect("Not a valid string");
    let value:i32 = value.trim().parse().expect("Not a valid value");

    for x in lower_bound..upper_bound {
        let product = value * x;
        println!("{}", product);
    }
   }
}
