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
    fn percentage_leverage(&self) -> u32 {
        let assets = self.company_shares;
        let liabilities = self.company_liabilities;
        ((assets - liabilities) / assets) * 100
    }
}

fn main() {
        //Making the Springate Model(Z-score) table for the checker

        println!("\nSpringate Model(Z-score).");

        println!("\n\t\tCOMPANY\t\t\tCOMPANY'S SHARES(N)\tCOMPANY LIABILITIES(N)\n\t\tCadburry Nigeria Plc\t\t15,000,000\t5,500,000
            Champion Breweries Plc\t\t25,000,000\t8,000,000
            Dangote Sugar Refinery Plc\t\t18,000,000\t10,000,000
            Flour Mills Nigeria Plc\t\t32,000,000\t4,000,000
            Nestle Nigeria Plc\t\t\t8,000,000\t1,500,000
            Unilever Nigeria Plc\t\t37,000,000\t11,000,000
            Honeywell Nigeria Plc\t\t34,000,000\t9,000,000
            Nigerian Breweries Plc\t\t30,000,000\t12,000,000");

    //Create a company vector to store details.

    let mut company_vector = Vec::new();
    
    let comp1 = Company {
        company_name:String::from("Cadburry Nigeria Plc"),
        company_shares:15000000,
        company_liabilities:5500000,
        date_founded:1965
    };
    company_vector.push(comp1);

    let comp2 = Company {
        company_name:String::from("Champion Breweries Plc"),
        company_shares:25000000,
        company_liabilities:8000000,
        date_founded:1974
    };
    company_vector.push(comp2);

    let comp3 = Company {
        company_name:String::from("Dangote Sugar Refinery Plc"),
        company_shares:18000000,
        company_liabilities:10000000,
        date_founded:1970
    };
    company_vector.push(comp3);

    let comp4 = Company {
        company_name:String::from("Flour Mills Nigeria Plc"),
        company_shares:32000000,
        company_liabilities:4000000,
        date_founded:1960
    };
    company_vector.push(comp4);

    let comp5 = Company {
        company_name:String::from("Nestle Nigeria Plc"),
        company_shares:8000000,
        company_liabilities:1500000,
        date_founded:1961
    };
    company_vector.push(comp5);

    let comp6 = Company {
        company_name:String::from("Unilever Nigeria Plc"),
        company_shares:37000000,
        company_liabilities:11000000,
        date_founded:1923
    };
    company_vector.push(comp6);

    let comp7 = Company {
        company_name:String::from("Honeywell Nigeria Plc"),
        company_shares:34000000,
        company_liabilities:9000000,
        date_founded:1906
    };
    company_vector.push(comp7);

    let comp8 = Company {
        company_name:String::from("Nigerian Breweries Plc"),
        company_shares:30000000,
        company_liabilities:12000000,
        date_founded:1946
    };
    company_vector.push(comp8);
    
    for company in &company_vector {
    
    let mut username = String::new();
    println!("\nWhat is the Company's Username? {}", username);
    io::stdin().read_line(&mut username).expect("Not a valid string");
    let username = username.trim();

        if username != "Cadb" && "Cadb".len() < 3 && "Cadb".len() > 8 
        || username != "Cham" && "Cham".len() < 3 && "Cham".len() > 8
        || username != "Dang" && "Dang".len() < 3 && "Dang".len() > 8
        || username != "Flou" && "Flou".len() < 3 && "Flou".len() > 8
        || username != "Nest" && "Nest".len() < 3 && "Nest".len() > 8
        || username != "Unil" && "Unil".len() < 3 && "Unil".len() > 8
        || username != "Hone" && "Hone".len() < 3 && "Hone".len() > 8
        || username != "Nige" && "Nige".len() < 3 && "Nige".len() > 8 
        {
         println!("Error!, Input does not match expected Input");
         break;
        }

    let mut password = String::new();
    println!("\nEnter Company Password. {}", password);
    io::stdin().read_line(&mut password).expect("Not a valid string");
    let password:&str = password.trim();

    //check if password meets requirements
    // #Checks if the password contains lowercase letters(a-z) and numbers(0-9)
    if password.chars().all(|c| c.is_ascii_lowercase() || c.is_numeric()) {
        println!("Password must only consist of lowercase letters(a-z) and numbers(0-9)");
        continue;
    }

    // #Checks if the password contains the prohibited special character symbols.
    //the contain() checks if a string(password) contains a particular sub_string(special characters)
    if password.contains(&"$") || password.contains(&"#") || password.contains(&"@") {
        println!("\nCompany password cannot comprise $ or # or @");
        continue;
    }

    if password.len() < 1 {
        println!("\nThe password length must not be less than 1");
    }


    if username == "Cadb" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());      
    }

    else if username == "Cham" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Dang" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Flou" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Nest" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Unil" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Hone" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else if username == "Nige" {
     println!("\t\t\tSpringate Model(Z-score)\n");
     println!("Company: {}\nDate founded: {}\tShares: {}\tLiabilities: {}\tPercentage Leverage: {}\t",
     company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage());   
    }

    else {
     println!("\nError!,Could not display data due to wrong Input");
    }
    
    display(&company_vector);

    }

}

    //Writing Company Data to a File
fn display(company_vector:&Vec<Company>) {

    use std::io::Write;

    use std::fs::File;

    let mut file = File::create("company.txt").expect("Failed to Create File");

    file.write_all("\t\t\tSpringate Model(Z-score)\n".as_bytes()).expect("Failed to Write");
    file.write_all("\t\t\t--------\n\n".as_bytes()).expect("Failed to Write");

    for company in company_vector.iter() {
        if company.company_shares > 20000000 {
         let  multi_perlev = (1 / 100) * company.percentage_leverage();
         writeln!(file,"Company\tDate founded\tAssets\tLiabilities\tPercentage Leverage\n{}\t{}\t{}\t{}\t{}",
         company.company_name,company.date_founded,company.company_shares,company.company_liabilities,multi_perlev).expect("Failed to write to file");
        }

        else if company.company_liabilities < 10000000 {
         let five_percent = (5 / 100) * company.percentage_leverage();
         writeln!(file,"Company\tDate founded\tAssets\tLiabilities\tPercentage Leverage\n{}\t{}\t{}\t{}\t{}",
         company.company_name,company.date_founded,company.company_shares,company.company_liabilities,five_percent).expect("Failed to write to file");
        }

        else {
         writeln!(file,"Company\tDate founded\tAssets\tLiabilities\tPercentage Leverage\n{}\t{}\t{}\t{}\t{}",
         company.company_name,company.date_founded,company.company_shares,company.company_liabilities,company.percentage_leverage()).expect("Failed to write to file");
        }

    }

}

















    







       
    




