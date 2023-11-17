//Rust program for a students council voting system for Pan-Atlantic University.

use std::io;

fn main() {
    loop {
        let mut input1 = String::new();
        println!("\nEnter the total number of eligible candidates: {}", input1);
        io::stdin().read_line(&mut input1).expect("Not a valid string");
        let total_candidates:f32 = input1.trim().parse().expect("Not a valid number");

        if total_candidates > 150.0 {
            break
        }else {
            println!("\nWelcome citizen, thank you for coming to vote today!");
         }

         let mut name = String::new();
         println!("\nWhat is your name? {}", name);
         io::stdin().read_line(&mut name).expect("Nota a valid name");
         let name = name.trim();

         let mut email = String::new();
         println!("\nEnter your email address {}", email);
         io::stdin().read_line(&mut email).expect("Not a valid email address");
         let email = email.trim();

         let mut department = String::new();
         println!("\nEnter your department {}", department);
         io::stdin().read_line(&mut department).expect("Not a valid department");
         let department = department.trim();

         let mut p = String::new();
         println!("\nEnter your state of origin {}", p);
         io::stdin().read_line(&mut p).expect("Not a valid state of origin");
         let p = p.trim();

         let mut a = String::new();
         println!("\nAre you a class rep? Y/N");
         io::stdin().read_line(&mut a).expect("Not a valid string");
         let a = a.trim();

         let mut input2 = String::new();
         println!("\nAre you in 100 level? Y/N");
         io::stdin().read_line(&mut input2).expect("Not a valid string");
         let level = input2.trim();

         let mut cgpa = String::new();
         println!("\nWhat is your current CPGA {}", cgpa);
         io::stdin().read_line(&mut cgpa).expect("Not a valid string");
         let cgpa:f32 = cgpa.trim().parse().expect("Not a valid number");

         if a == "Y" && level == "N" && cgpa > 4.0 && cgpa < 5.1 {
         println!("candidate name is: {}
                   candidate email is: {}
                   candidate department is: {}
                   candidate state of origin is: {}
                   ", name,email,department,p);
         println!("\nYou can vote!");
        }else {
        println!("Sorry, you are not eligible to vote");
         }           


    }
}
