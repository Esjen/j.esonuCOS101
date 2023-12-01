struct Staff {
    profession: String,
    _years_of_experience: u8,
}

fn validate_staff_level(staff: &Staff) -> Option<&'static str> {
    let validation_table: Vec<(&str, &str)> = vec![
        ("Public Servant", "APS 1-2"),
        ("Office Administrator", "APS 3-5"),
        ("Academic", "APS 5-8"),
        ("Lawyer", "APS 5-8"),
        ("Teacher", "APS 5-8"),
        ("Intern", "APS 1-2"),
        ("–", "APS 3-5"),
        ("Paralegal", "APS 3-5"),
        ("Placement", "APS 3-5"),
        ("Administrator", "APS 3-5"),
        ("Research Assistant", "APS 3-5"),
        ("Junior Associate", "APS 5-8"),
        ("Classroom Teacher", "APS 5-8"),
        ("Senior Administrator", "APS 5-8"),
        ("PhD Candidate", "APS 5-8"),
        ("Associate", "APS 5-8"),
        ("Snr Teacher", "APS 5-8"),
        ("Office Manager", "EL1 8-10"),
        ("Post-Doc Researcher", "EL1 8-10"),
        ("Senior Associate 1-2", "EL1 8-10"),
        ("Leading Teacher", "EL1 8-10"),
        ("Director", "EL2 10-13"),
        ("Senior Lecturer", "EL2 10-13"),
        ("Senior Associate 3-4", "EL2 10-13"),
        ("Deputy Principal", "EL2 10-13"),
        ("SES", "SES"),
        ("CEO", "SES"),
        ("Dean", "SES"),
        ("Partner", "SES"),
        ("Principal", "SES"),
    ];

    for entry in validation_table {
        if staff.profession == entry.0 {
            return Some(entry.1);
        }
    }
    None
}

fn main() {
    // Taking user input for the staff's profession and years of experience
    println!("Enter the staff's profession:");
    let mut profession = String::new();
    std::io::stdin().read_line(&mut profession).expect("Failed to read line");
    let profession = profession.trim().to_string();

    println!("Enter the number of years of work experience:");
    let mut years_exp = String::new();
    std::io::stdin().read_line(&mut years_exp).expect("Failed to read line");
    let years_exp: u8 = years_exp.trim().parse().expect("Please enter a valid number");

    let staff_member = Staff {
        profession,
        _years_of_experience: years_exp,
    };

    if let Some(level) = validate_staff_level(&staff_member) {
        println!("The staff member holds position: {}", level);
    } else {
        println!("Unable to determine the staff member's position.");
    }
}
