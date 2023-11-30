use std::io;

struct Sibling {
    name: String,
    age: u8,
    marital_status: Option<String>,
    offspring: Option<u8>,
    city: Option<String>,
    school_attended: Option<String>,
    class_level: Option<String>,
    university: Option<String>,
    course_of_study: Option<String>,
}

fn main() {
    let mut siblings: Vec<Sibling> = Vec::new();

    println!("How many siblings do you have?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_siblings: usize = input.trim().parse().expect("Please enter a valid number");

    for _ in 0..num_siblings {
        let mut sibling = Sibling {
            name: String::new(),
            age: 0,
            marital_status: None,
            offspring: None,
            city: None,
            school_attended: None,
            class_level: None,
            university: None,
            course_of_study: None,
        };

        println!("Enter sibling's name:");
        io::stdin().read_line(&mut sibling.name).expect("Failed to read input");

        println!("Enter sibling's age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        sibling.age = age_input.trim().parse().expect("Please enter a valid age");

        if sibling.age > 18 {
            println!("Is {} married? (yes/no)", sibling.name);
            let mut marital_status_input = String::new();
            io::stdin().read_line(&mut marital_status_input).expect("Failed to read input");
            if marital_status_input.trim().to_lowercase() == "yes" {
                sibling.marital_status = Some("Married".to_string());
                println!("How many offspring do they have?");
                let mut offspring_input = String::new();
                io::stdin().read_line(&mut offspring_input).expect("Failed to read input");
                sibling.offspring = Some(offspring_input.trim().parse().expect("Please enter a valid number"));

                println!("What city do they live in?");
                let mut city_input = String::new();
                io::stdin().read_line(&mut city_input).expect("Failed to read input");
                sibling.city = Some(city_input.trim().to_string());
            } else {
                sibling.marital_status = Some("Single".to_string());
                println!("Is {} a student or a worker?", sibling.name);
                let mut status_input = String::new();
                io::stdin().read_line(&mut status_input).expect("Failed to read input");
                if status_input.trim().to_lowercase() == "student" {
                    println!("Which university does {} attend?", sibling.name);
                    let mut university_input = String::new();
                    io::stdin().read_line(&mut university_input).expect("Failed to read input");
                    sibling.university = Some(university_input.trim().to_string());

                    println!("What course is {} studying?", sibling.name);
                    let mut course_input = String::new();
                    io::stdin().read_line(&mut course_input).expect("Failed to read input");
                    sibling.course_of_study = Some(course_input.trim().to_string());
                }
            }
        } else {
            println!("Has {} written WAEC? (yes/no)", sibling.name);
            let mut waec_input = String::new();
            io::stdin().read_line(&mut waec_input).expect("Failed to read input");
            if waec_input.trim().to_lowercase() == "yes" {
                println!("Which secondary school did {} attend?", sibling.name);
                let mut school_input = String::new();
                io::stdin().read_line(&mut school_input).expect("Failed to read input");
                sibling.school_attended = Some(school_input.trim().to_string());
            } else {
                println!("What is {}'s current class level?", sibling.name);
                let mut class_input = String::new();
                io::stdin().read_line(&mut class_input).expect("Failed to read input");
                sibling.class_level = Some(class_input.trim().to_string());
            }
        }

        siblings.push(sibling);
    }

    // Displaying sibling data
    println!("Sibling Information:");
    for sibling in &siblings {
        println!("Name: {}", sibling.name);
        println!("Age: {}", sibling.age);

        if let Some(marital_status) = &sibling.marital_status {
            println!("Marital Status: {}", marital_status);
            if marital_status == "Single" {
                if let Some(status) = &sibling.university {
                    println!("University: {}", status);
                }
                if let Some(course) = &sibling.course_of_study {
                    println!("Course of Study: {}", course);
                }
            } else {
                if let Some(offspring) = &sibling.offspring {
                    println!("Number of Offspring: {}", offspring);
                }
                if let Some(city) = &sibling.city {
                    println!("City: {}", city);
                }
            }
        } else {
            if let Some(school) = &sibling.school_attended {
                println!("School Attended: {}", school);
            } else if let Some(class_level) = &sibling.class_level {
                println!("Class Level: {}", class_level);
            }
        }

        println!("----------------------");
    }
}
