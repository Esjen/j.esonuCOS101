//Rust program to develop a Researchers Publication Incentive System

use std::io;

fn main() {
    loop {
        let mut input1 = String::new();
        println!("\nWhat is the Total Number of Researchers? {}", input1);
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let tot_researchers:f32 = input1.trim().parse().expect("Not a valid number");

        if tot_researchers > 500.0 {
            break
        }else {
            println!("\nWelcome, I am here to tell you your incentive!");
         }

         let mut _name = String::new();
         println!("\nWhat is your name? {}", _name);
         io::stdin().read_line(&mut _name).expect("Not a valid name");
         let _name = _name.trim();

         let mut input2 = String::new();
         println!("\nwhat is the total number of papers you have published? {}", input2);
         io::stdin().read_line(&mut input2).expect("Not a valid string");
         let no_of_papers:f32 = input2.trim().parse().expect("Not a valid number");

         if no_of_papers == 3.0 && no_of_papers == 4.0 && no_of_papers == 5.0 
         {
           println!("\nYOUR INCENTIVE IS N500,000");
         }
         else if no_of_papers > 5.0 && no_of_papers < 10.0 
         {
            println!("\nYOUR INCENTIVE IS N800,000");
         }
         else if no_of_papers >= 10.0 
         {
            println!("\nYOUR INCENTIVE IS N1,000,000.0");
         }
         else if no_of_papers < 3.0 
         {
            println!("\nYOUR INCENTIVE IS N100,000.0");
         }
         else 
         {
            println!("\nYOU HAVE NO INCENTIVE");
         }   

             let mut e = String::new();
             println!("/nDo you want to continue?, Y/N");
             io::stdin().read_line(&mut e).expect("Not a valid string");
             let e = e.trim();
             if e == "N" {
               break
             }

    }



}
