//Rust program to to check requirements
//Implementing using Rust Structs and Vectors

use std::io;


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

    for _x in companies {

    for _leverages in 0..=percentage_leverages.len() {

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
    }
    }
    borrow_vector(&companies,&percentage_leverages);
}

    //Writing Company Data to a File
fn borrow_vector(companies:&Vec<Company>,percentage_leverages:&Vec<f32>) {

    use std::fs::File;
    use std::io::Write;

     // Create a file to store company data and leverage percentages
    let mut file = File::create("company_data.txt").expect("Failed to create file.");

    file.write_all("\t\t\tSpringate Model(Z score)\n".as_bytes()).expect("Failed to Write");
    file.write_all("\t\t\t--------\n\n".as_bytes()).expect("Failed to write");

    // Write header to company file
    writeln!(file, "Company Name\tDate Founded\tAssets\tLiabilities\tLeverage Percentage").expect("Failed to write to file.");

    // Create vectors to store leverages for conditions
    let mut high_shares_leverages = Vec::new();
    let mut low_liability_5_percent_leverages = Vec::new();

    for company in companies {
        for k in 0..=7 {
        let leverage = percentage_leverages[k];
        writeln!(file, "{}\t{}\t{}\t{}\t{:.2}%", company.company_name, company.date_founded, company.company_shares, company.company_liabilities, leverage).expect("Failed to write to file.");

        if company.company_shares > 20000000 {
            let multi_perlev = leverage * 0.01;
            high_shares_leverages.push(multi_perlev); // Save multiple of leverage for companies with shares > 20,000,000
        }

        if company.company_liabilities < 10000000 {
            let five_percent = leverage * 0.05;
            low_liability_5_percent_leverages.push(five_percent); // Save 5% of leverage for companies with liabilities < 10,000,000
        }
        }
    }

    // Create a file to store multiple of leverage for high shares companies
    let mut high_shares_leverage_file = File::create("high_shares_leverages.txt").expect("Failed to create file.");
    for leverage in &high_shares_leverages {
        writeln!(high_shares_leverage_file, "{:.2}%", leverage).expect("Failed to write to file.");
    }

    // Create a file to store 5% of leverage for low liability companies
    let mut low_liability_5_percent_leverage_file = File::create("low_liability_5_percent_leverages.txt").expect("Failed to create file.");
    for leverage in &low_liability_5_percent_leverages {
        writeln!(low_liability_5_percent_leverage_file, "{:.2}%", leverage).expect("Failed to write to file.");
    }
    checker(companies,percentage_leverages);
}







       
    





