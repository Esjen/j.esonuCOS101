use std::io;

fn main() {
    // Input phase: User enters the number of developers
    println!("Enter the number of developers:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num_developers: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let mut developers: Vec<(String, u32)> = Vec::new();

    // Input phase: User enters developer names and years of experience
    for i in 0..num_developers {
        println!("Enter name of developer {}:", i + 1);
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");

        println!("Enter years of experience for developer {}:", i + 1);
        let mut experience_input = String::new();
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read input");

        let experience: u32 = match experience_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number for years of experience.");
                return;
            }
        };

        developers.push((name.trim().to_string(), experience));
    }

    // Finding developer with the highest years of experience
    let mut max_experience = 0;
    let mut max_experience_developer = ("No developer".to_string(), 0);

    for dev in &developers {
        if dev.1 > max_experience {
            max_experience = dev.1;
            max_experience_developer = dev.clone();
        }
    }

    // Displaying developer with the highest years of experience
    if max_experience > 0 {
        println!(
            "The developer with the highest years of experience is {} with {} years.",
            max_experience_developer.0, max_experience_developer.1
        );
    } else {
        println!("No developers found.");
    }
}
