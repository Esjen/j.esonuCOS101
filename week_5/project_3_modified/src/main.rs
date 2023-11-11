//Rust program to display the menu and prices of various food items 
//as well as calculate the total prices based on the order placed

use std::io;

fn main() {
    println!("WELCOME TO JOHNS FOOD SERVICES!
  ITEM NO.                       MENU                     PRICE(FOR ONE PORTION)
  1                 P = Poundo Yam/Edinkaiko Soup               - N3,200
  2                 F = Fried Rice & Chicken                    - N3,000
  3                 A = Amala & Ewedu Soup                      - N2,500
  4                 E = Eba & Egusi Soup                        - N2,000
  5                 W = White Rice & Stew                       - N2,500

                   NOTE: CUSTOMERS WITH ORDERS EXCEEDING N10000 GET A 5% DISCOUNT!
    ");
    loop {
    let pp:f32 = 3200.0;
    let pf:f32 = 3000.0;
    let pa:f32 = 2500.0;
    let pe:f32 = 2000.0;
    let pw:f32 = 2500.0;

    let p = "Pounded Yam/Edikainko Soup";
    let f = "Fried Rice & Chicken";
    let a = "Amala and Ewedu Soup";
    let e = "Eba and Egusi Soup";
    let w = "White Rice and Stew";

    println!("PLEASE ENTER THE ITEM NO. OF WHAT YOU WANT TO ORDER");
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Invalid string");
    let i = i.trim();

    if i == "1"{

    println!("\nHow many portions of {} would you like to order?", p);
    }

    else if i == "2"{
    println!("\nHow many portions of {} would you like to order?", f); 
    }

    else if i == "3"{
    println!("\nHow many portions of {} would you like to order?", a);
    }

    else if i == "4"{
    println!("\nHow many portions of {} would you like to order?", e);
    }

    else if i == "5" {
    println!("\nHow many portions of {} would you like to order?", w);
    }

    else {
    println!("\nThe item number you picked does not exist on the menu. Pick another");continue;
    }

    let mut q = String::new();
    io::stdin().read_line(&mut q).expect("Not a valid string");
    let q:f32 = q.trim().parse().expect("Not a valid number");


    if i == "1"{let t = pp * q;
        println!("\nThe total cost of your order is N{}", t);
    }

    else if i == "2"{let t = pf * q;
        println!("\nThe total cost of your order is N{}", t);
    }
    
    else if i == "3"{let t = pa * q;
        println!("\nThe total cost of your order is N{}", t);
    }
    
    else if i == "4" {let t = pe * q;
        println!("\nThe total cost of your order is N{}", t);
    }
    
    else if i == "5" {let t = pw * q;
        println!("\nThe total cost of your order is N{}", t);
    }
    
    if t > 10000.0 {
    let dt = t - (0.05 * t);
    println!("\nThe price of your order exceeds N10000");
    println!("\nYou have received a discount of 5%");
    println!("\nYour total price is N{}", dt);
    println!("\nHave a nice day!, We hope to see you again");
    }
    }
}    


            



