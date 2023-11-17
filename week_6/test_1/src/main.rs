//Rust program to find patients information given certain data
use std::io;
fn main() {
    loop{

        let mut input1 = String::new();
        println!("\nWhat is the patients name: {}", input1);
        io::stdin().read_line(&mut name).expect("Not a valid name");
        let name = input1.trim();

        let mut input2 = String::new();
        println!("\nEnter your date of birth: {}",  input2);
        io::stdin().read_line(&mut input2).expect("Not a valid string");
        let dob:f32 = input2.trim().parse().expect("Not a valid date of birth");

        let mut input3 = String::new();
        println!("\nEnter your email address: {}", input3);
        io::stdin().read_line(&mut input3).expect("Not a valid email address");
        let email_address = input3.trim();

        let mut input4 = String::new();
        println!("\nEnter your phone number: {}", input4);
        io::stdin().read_line(&mut input4).expect("Not a valid string");
        let phone_number:f32 = input4.trim().parse().expect("Not a a valid phone number");

        let mut input5 = String::new();
        println!("\nEnter the number of siblings that you have {}", input5);
        io::stdin().read_line(&mut input5).expect("Not a valid string");
        let no_of_siblings:f32 = input5.trim().parse().expect("Not a valid number");

        let mut input6 = String::new();
        println!("\nEnter the number of children: {}", input6);
        io::stdin().read_line(&mut input6).expect("Not a valid string");
        let no_of_children:f32 = input6.trim().parse().expect("Not a valid number");

        let mut input7 = String::new();
        println!("\nEnter your medical diagnosis: {}", input7);
        io::stdin().read_line(&mut input7).expect("Not a valid medical diagnosis");
        let medical_diagnosis = input7.trim();

        let mut input8 = String::new();
        println!("\nEnter your village of residence {}", input8);
        io::stdin().read_line(&mut input8).expect("Not a valid village");
        let village_of_residence = input8.trim();

        let mut age = String::new();
        println!("\nEnter your age(in years): {}", age);
        io::stdin().read_line(&mut age).expect("Not a valid string");
        let age:f32 = age.trim().parse().expect("Not a valid age");

        let mut m = "medical_diagnosis";
        let mut d = "dob";
        let mut n = "name";
        let mut e = "email_address";
        let mut h = "phone_number";
        let mut s ="no_of_siblings";
        let mut c = "no_of_children";
        let mut v = "village_of_residence";

        let total_number = String::new();
        println!("\nWhat is the total number of patients? {}", total_number);
        io::stdin().read_line(&mut total_number).expect(not a valid string);
        let total_number:f32 = total_number.trim().parse().expect("Not a valid number");

        if total_number == 100.0 && total_number < 100.0 {
            continue;
        }else  {
            println!("\nThe total number of patients have been exceeded. come next time");
        }



        if m = "alzheimer" && age > 50 && v == "Akpabom" && c > "4" {
            let a:f32 = 1200000.0;
            let b:f32 = a - (20 / 100);
            println!("\nThe total price is(in naira) {}", b);
        }else{ 
            println!("\nYour total price is {}", a);
         }

        
       if m = "Arrythmia" && age = 30 && s > "4" && v == "Ngbauji" {
            let r:f32 = 550,000.0;
            let w:f32 = r - (5 / 100);
            println!("\nThe total price is(in naira) {}", w);
       }else {
            println!("\nYour total price is {}", r);
        }

        

        if m = "chronic kidney disease" && age > 40 && s > "3" && c > "3" && v == "Atabrikang" {
            let h:f32 = 1500000.0;
            let q:f32 = h - (15 / 100);
            println!("\nYour total price is(in naira) {}", q);
        }else {
            println!("\nYour total price is {}", h);
         }

        if m = "diabetes" && age < 28 && age < 45 && c = "2..4" && v == "okorobilom" {
            let u:f32 = 800,000.0;
            let e:f32 = u - (10 / 100);
            println!("\nYour total price is(in naira) {}", e);
        }else {
            println!("\nYour total price is {}", u);
         }   

            if m = "Arthritis"&& age > 58 && c > 5 && s > 5 && v == "emerenem" { 
                let t:f32 = 450000.0;
                let y:f32 = t - (10 / 100);
                println!("your total price is(in naira) {}", y);
            }else {
                println!("\nYour total price is {}", t);
             }

             let mut o = String::new();
             println!("\nDo you want to continue?, Y/N");
             io::stdin().read_line(&mut o).expect("Not a valid string");
             let o = o.trim();
             if o == "N" {
             break;
             }
    }
} 


             



            





