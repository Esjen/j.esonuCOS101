//Rust program that determines the annual incentive 
//of an employee given his experience and age

use std::io;

fn main() {
    loop{

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nAre you experienced? Y/N");
    io::stdin().read_line(&mut input1).expect("Not a valid experience");
    let input1 = input1.trim();

    println!("\nEnter your Age (in years):");
    io::stdin().read_line(&mut input2).expect("Not a valid age");
    let age:f32 = input2.trim().parse().expect("Not a valid number");

    if input1 == "Y" && age >= 40.0
    {
        println!("Your Annual incentive is N1,560,000.0");
    }
    else if input1 == "Y" && age >= 30.0 && age < 40.0
    {
        println!("Your Annual incentive is N1,480,000.0");
    }
    else if input1 == "Y" && age < 28.0
    {
        println!("Your Annual incentive is N1,300,000.0");
    }
    else 
    {
        println!("Your Annual incentive is N100,000");
    }
    
    
    let mut e = String::new();
    println!("\nDo you want to proceed? Y/N");
    io::stdin().read_line(&mut e).expect("Not a valid string");
    let e = e.trim();
    if e == "N" {
     break
    }
    }
}      


    

    

