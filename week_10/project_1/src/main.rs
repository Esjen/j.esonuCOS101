use std::io;

// Define a struct for representing laptop details
struct Laptop {
    brand: String,
    price: u64,
    available_quantity: u64,
}

impl Laptop {
    // Method to calculate the total cost for a specific number of laptops
    fn calculate_total_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    // Create instances of laptops
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650_000,
            available_quantity: 10,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755_000,
            available_quantity: 6,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550_000,
            available_quantity: 10,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850_000,
            available_quantity: 4,
        },
    ];

    // Prompt user input for the types of laptops they want to buy
    println!("Enter the types of laptops you want to buy (HP, IBM, Toshiba, Dell):");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let selected_laptops: Vec<&str> = user_input.split_whitespace().collect();

    // Calculate total cost for purchasing three laptops from each selected brand
    let mut total_cost: u64 = 0;

    for laptop in laptops {
        if selected_laptops.contains(&&*laptop.brand) {
            let quantity_to_buy = 3.min(laptop.available_quantity);
            total_cost += laptop.calculate_total_cost(quantity_to_buy);
        }
    }

    // Display the total cost for the selected laptops
    println!("Total cost for 3 laptops from each selected brand: ₦{}", total_cost);
}
