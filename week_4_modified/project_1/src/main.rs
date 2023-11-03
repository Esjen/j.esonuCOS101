//Rust program to compute the speed of a car given its distance and time

use std::io;

fn main() {
    loop{ 

    let mut d = String::new();
    let mut t = String::new();

    println!("\nWhat is the distance covered by the car (in miles)?");
    io::stdin().read_line(&mut d).expect("Not a valid string");
    let mut d:f64 = d.trim().parse().expect("Not a valid distance");
    d = d * 1.609344;

    println!("\nHow long did it take the car to cover that distance(in hours)?");
    io::stdin().read_line(&mut t).expect("Not a valid string");
    let t:f64 = t.trim().parse().expect("Not a valid time");

    let speed:f64 = d / t;
    
    println!("\nThe speed of the car is {}km/h", speed);

    let mut q = String::new();
    println!("\nDo you want to continue? Y / N");
    io::stdin().read_line(&mut q).expect("Not a valid string");
    let q = q.trim();
    if q == "N" {
        break
    }
}     







    
}
