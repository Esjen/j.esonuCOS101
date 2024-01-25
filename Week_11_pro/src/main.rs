//Rust program to to check requirements
//Implementing using Rust Structs and Vectors

use std::io;
use std::fs::File;
use std::io::Write;


struct Company {
    company_name:String,
    company_shares:u32,
    company_liabilities:u32,
    date_founded:u32
}

impl Company {
    fn percentage_leverage(&self) -> f32 {
        let assets = self.company_shares as f32;
        let liabilities = self.company_liabilities as f32;
        ((assets - liabilities) / assets) * 100.0
    }
}

fn main() {
    //Making the Springate Model(Z-score) table for the checker

    //Create a leverage vector to store company leverages.
    //create a company vector to store company details
let mut percentage_leverages: Vec<f32> = Vec::new();
    
    let companies = vec![
        Company {
        company_name:String::from("Cadburry Nigeria Plc"),
        company_shares:15000000,
        company_liabilities:5500000,
        date_founded:1965
        },
        Company {
        company_name:String::from("Champion Breweries Plc"),
        company_shares:25000000,
        company_liabilities:8000000,
        date_founded:1974
        },
        Company {
        company_name:String::from("Dangote Sugar Refinery Plc"),
        company_shares:18000000,
        company_liabilities:10000000,
        date_founded:1970
        },
        Company {
        company_name:String::from("Flour Mills Nigeria Plc"),
        company_shares:32000000,
        company_liabilities:4000000,
        date_founded:1960
        },
        Company {
        company_name:String::from("Nestle Nigeria Plc"),
        company_shares:8000000,
        company_liabilities:1500000,
        date_founded:1961
        },
        Company {
        company_name:String::from("Unilever Nigeria Plc"),
        company_shares:37000000,
        company_liabilities:11000000,
        date_founded:1923
        },
        Company {
        company_name:String::from("Honeywell Nigeria Plc"),
        company_shares:34000000,
        company_liabilities:9000000,
        date_founded:1906
        },
        Company {
        company_name:String::from("Nigerian Breweries Plc"),
        company_shares:30000000,
        company_liabilities:12000000,
        date_founded:1946
        },
    ];
    for company in &companies {
        percentage_leverages.push(company.percentage_leverage());
    }
    checker(&companies,&percentage_leverages);
}

fn password_checker(password: &str) -> bool {
    // Check for uppercase letters
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        return false;
    }

    // Check for specific special characters
    if password.contains('$') || password.contains('#') || password.contains('@') {
        return false;
    }

    // Password passed all checks
    true
}


fn checker(companies:&Vec<Company>,percentage_leverages:&Vec<f32>) {

    for company in companies {

    for _k in 0..=percentage_leverages.len() {

    let mut password = String::new();
    println!("\nEnter Company Password. {}", password);
    io::stdin().read_line(&mut password).expect("Not a valid string");
    let password = password.trim();

    //check if password meets requirements
    // #Checks if the password contains lowercase letters(a-z) and numbers(0-9)

    if password.len() < 1 {
        println!("\nThe password length must not be less than 1");
    }
    else if password_checker(password) {
        println!("The password is valid");
    }
    else {
        println!("Password must not comprise Uppercase letters or the special characters $, # or @ ");
        break
    }

    let mut username = String::new();
    println!("\nWhat is the Company's Username? {}", username);
    io::stdin().read_line(&mut username).expect("Not a valid string");
    let username = username.trim();

        let username_vector = vec!["Cadb","Cham","Dang","Flou","Nest","Unil","Hone","Nige"];

        for x in 0..username_vector.len() {
            if username != username_vector[x] && username_vector[x].len() < 3 && username_vector[x].len() > 8 {
            println!("Error!, Invalid Input");
            break;
        }

            if  username == "Cadb" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[0].company_name,companies[0].date_founded,companies[0].company_shares,companies[0].company_liabilities,percentage_leverages[0]);   
            }

            else if username == "Cham" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[1].company_name,companies[1].date_founded,companies[1].company_shares,companies[1].company_liabilities,percentage_leverages[1]);   
            }

            else if username == "Dang" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[2].company_name,companies[2].date_founded,companies[2].company_shares,companies[2].company_liabilities,percentage_leverages[2]);   
            }

            else if username == "Flou" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[3].company_name,companies[3].date_founded,companies[3].company_shares,companies[3].company_liabilities,percentage_leverages[3]);   
            }

            else if username == "Nest" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[4].company_name,companies[4].date_founded,companies[4].company_shares,companies[4].company_liabilities,percentage_leverages[4]);   
            }

            else if username == "Unil" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[5].company_name,companies[5].date_founded,companies[5].company_shares,companies[5].company_liabilities,percentage_leverages[5]);   
            }

            else if username == "Hone" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[6].company_name,companies[6].date_founded,companies[6].company_shares,companies[6].company_liabilities,percentage_leverages[6]);   
            }

            else if username == "Nige" {
            println!("\t\t\tSpringate Model(Z-score)\n");
            println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}%\t",
            companies[7].company_name,companies[7].date_founded,companies[7].company_shares,companies[7].company_liabilities,percentage_leverages[7]);   
            }

            else {
                println!("You have no company data available due to wrong input");
            }


        }

    //Writing Company Data to a File


    // Create a file to store company data and leverage percentages
    let mut file = File::create("company_data.txt").expect("Failed to create file.");

    file.write_all("\t\t\tSpringate Model(Z score)\n".as_bytes()).expect("Failed to Write");
    file.write_all("\t\t\t--------\n\n".as_bytes()).expect("Failed to write");
            
     writeln!(file,"\n\tCompany\t\t\tDate Founded\t\tShares\t\t\tLiabilities\tLeverage(%)\n\tCadburry Nigeria Plc\t\t1965\t\t{}\t\t{}\t\t{}
      Champion Breweries Plc\t\t1974\t\t{}\t\t{}\t\t{}
      Dangote Sugar Refinery Plc\t1970\t\t{}\t\t{}\t{}
      Flour Mills Nigeria Plc\t\t1960\t\t{}\t\t{}\t\t{}
      Nestle Nigeria Plc\t\t1961\t\t{}\t\t\t{}\t\t{}
      Unilever Nigeria Plc\t\t1923\t\t{}\t\t{}\t{}
      Honeywell Nigeria Plc\t\t1906\t\t{}\t\t{}\t\t{}
      Nigerian Breweries Plc\t\t1960\t\t{}\t\t{}\t{}",companies[0].company_shares,companies[0].company_liabilities,percentage_leverages[0],
      companies[1].company_shares,companies[1].company_liabilities,percentage_leverages[1],
      companies[2].company_shares,companies[2].company_liabilities,percentage_leverages[2],
      companies[3].company_shares,companies[3].company_liabilities,percentage_leverages[3],
      companies[4].company_shares,companies[4].company_liabilities,percentage_leverages[4],
      companies[5].company_shares,companies[5].company_liabilities,percentage_leverages[5],
      companies[6].company_shares,companies[6].company_liabilities,percentage_leverages[6],
      companies[7].company_shares,companies[7].company_liabilities,percentage_leverages[7]).expect("Failed to write to file");


    if company.company_shares > 20000000 {
    let mut multi_lev_file = File::create("Multiple_leverage.txt").expect("Failed to write");
    let multiple_of_leverage_0 = 0.01 * percentage_leverages[0];
    let multiple_of_leverage_1 = 0.01 * percentage_leverages[1];
    let multiple_of_leverage_2 = 0.01 * percentage_leverages[2];
    let multiple_of_leverage_3 = 0.01 * percentage_leverages[3];
    let multiple_of_leverage_4 = 0.01 * percentage_leverages[4];
    let multiple_of_leverage_5 = 0.01 * percentage_leverages[5];
    let multiple_of_leverage_6 = 0.01 * percentage_leverages[6];
    let multiple_of_leverage_7 = 0.01 * percentage_leverages[7];

    writeln!(multi_lev_file,"\n\tCompany\t\t\tDate Founded\t\tShares\t\t\tLiabilities\tLeverage(%)\n\tCadburry Nigeria Plc\t\t1965\t\t{}\t\t{}\t\t{}
      Champion Breweries Plc\t\t1974\t\t{}\t\t{}\t\t{}
      Dangote Sugar Refinery Plc\t1970\t\t{}\t\t{}\t{}
      Flour Mills Nigeria Plc\t\t1960\t\t{}\t\t{}\t\t{}
      Nestle Nigeria Plc\t\t1961\t\t{}\t\t\t{}\t\t{}
      Unilever Nigeria Plc\t\t1923\t\t{}\t\t{}\t{}
      Honeywell Nigeria Plc\t\t1906\t\t{}\t\t{}\t\t{}
      Nigerian Breweries Plc\t\t1960\t\t{}\t\t{}\t{}",companies[0].company_shares,companies[0].company_liabilities,multiple_of_leverage_0,
      companies[1].company_shares,companies[1].company_liabilities,multiple_of_leverage_1,
      companies[2].company_shares,companies[2].company_liabilities,multiple_of_leverage_2,
      companies[3].company_shares,companies[3].company_liabilities,multiple_of_leverage_3,
      companies[4].company_shares,companies[4].company_liabilities,multiple_of_leverage_4,
      companies[5].company_shares,companies[5].company_liabilities,multiple_of_leverage_5,
      companies[6].company_shares,companies[6].company_liabilities,multiple_of_leverage_6,
      companies[7].company_shares,companies[7].company_liabilities,multiple_of_leverage_7).expect("Failed to write to file");
    }

    if company.company_liabilities < 10000000 {
    let mut five_percent_file = File::create("Five_percent.txt").expect("Failed to write to file");  
    let five_percent_0 = 0.05 * percentage_leverages[0];
    let five_percent_1 = 0.05 * percentage_leverages[1];
    let five_percent_2 = 0.05 * percentage_leverages[2];
    let five_percent_3 = 0.05 * percentage_leverages[3];
    let five_percent_4 = 0.05 * percentage_leverages[4];
    let five_percent_5 = 0.05 * percentage_leverages[5];
    let five_percent_6 = 0.05 * percentage_leverages[6];
    let five_percent_7 = 0.05 * percentage_leverages[7];
    writeln!(five_percent_file,"\n\tCompany\t\t\tDate Founded\t\tShares\t\t\tLiabilities\tLeverage(%)\n\tCadburry Nigeria Plc\t\t1965\t\t{}\t\t{}\t\t{}
      Champion Breweries Plc\t\t1974\t\t{}\t\t{}\t\t{}
      Dangote Sugar Refinery Plc\t1970\t\t{}\t\t{}\t{}
      Flour Mills Nigeria Plc\t\t1960\t\t{}\t\t{}\t\t{}
      Nestle Nigeria Plc\t\t1961\t\t{}\t\t\t{}\t\t{}
      Unilever Nigeria Plc\t\t1923\t\t{}\t\t{}\t{}
      Honeywell Nigeria Plc\t\t1906\t\t{}\t\t{}\t\t{}
      Nigerian Breweries Plc\t\t1960\t\t{}\t\t{}\t{}",companies[0].company_shares,companies[0].company_liabilities,five_percent_0,
      companies[1].company_shares,companies[1].company_liabilities,five_percent_1,
      companies[2].company_shares,companies[2].company_liabilities,five_percent_2,
      companies[3].company_shares,companies[3].company_liabilities,five_percent_3,
      companies[4].company_shares,companies[4].company_liabilities,five_percent_4,
      companies[5].company_shares,companies[5].company_liabilities,five_percent_5,
      companies[6].company_shares,companies[6].company_liabilities,five_percent_6,
      companies[7].company_shares,companies[7].company_liabilities,five_percent_7).expect("Failed to write to file");
    }

    println!("Done");
    }
    }
}

    

    
