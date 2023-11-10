// Rust program to compute the roots
// of a quadratic equation given
//values a,b, and c

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first coefficient: ");
    io::stdin().read_line(&mut input1).expect("Not a valid coefficient");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter Second coefficient: ");
    io::stdin().read_line(&mut input2).expect("Not a valid coefficient");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter Third coefficient: ");
    io::stdin().read_line(&mut input3).expect("Not a valid coefficient");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = (b * b) - 4.0 * a * c;
    
   
     let root1 = ((-b) + (d.sqrt())) / 2.0 * a;
     println!("The first root is: {}", root1);

     let root2 = ((-b) - (d.sqrt())) /2.0 * a;
     println!("The second root is: {}", root2);

    if d > 0.0 {
        println!("there are two distinct roots");
    }    

    else if d == 0.0 {
        println!("there is exactly one real root");
    }    

    else if d < 0.0 {
        println!("there are no real roots");
    }    
}
    

