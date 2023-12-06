use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // Creating a vector to store student details
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Displaying student details
    println!("Student Details:");
    for student in &students {
        println!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Writing student details to a file
    let file_name = "student_details.txt";
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to create file: {}", e);
        }
    };

    for student in &students {
        if let Err(e) = writeln!(
            file,
            "Student Name\tMatric . Number\tDepartment\tLevel\n{}\t{}\t{}\t{}",
            student.name, student.matric_number, student.department, student.level
        ) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    println!("Student details have been saved to {}.", file_name);
}
