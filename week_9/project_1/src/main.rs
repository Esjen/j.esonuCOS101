use std::fs::File;
use std::io::Write;

fn main() {
    // List of distinct high-quality drink categories
    let lager = vec![
          "33 export",
          "Desperados",
          "Goldberg",
          "Gulder",
          "Heineken",
          "Star",
    ];

    let stout = vec![
         "Legend",
         "Turbo drink",
         "Williams",
         "Blank",
         "Blank",
         "Blank",
    ];

    let non_alcoholic = vec![
         "Maltina",
         "Amstel Malt",
         "Malta Gold",
         "Fayrouz",
         "Blank",
         "Blank"
    ];

    // File creation and handling
    let file_name = "high_quality_drinks.txt";
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to create file: {}", e);
        }
    };

    // Writing the drink categories to the file
    for drink1 in &lager {
        if let Err(e) = writeln!(file, "{}", drink1) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    for drink2 in &stout {
        if let Err(e) = writeln!(file, "{}", drink2) {
            eprintln!("Failed to write to file: {}", e);
        }
    }

    for drink3 in &non_alcoholic {
        if let Err(e) = writeln!(file, "{}", drink3) {
            eprintln!("Failed to write to file: {}", e);
        }
    }


    println!("High-quality drink categories have been saved to {}.", file_name);
}
