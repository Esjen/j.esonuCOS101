// Define a struct for representing laptop details
struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    // Method to calculate the total cost for a specific number of laptops
    fn calculate_total_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    // Create instances of laptops
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };

    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };

    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };

    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Calculate total cost for purchasing three laptops from each brand
    let hp_total_cost = hp_laptop.calculate_total_cost(3);
    let ibm_total_cost = ibm_laptop.calculate_total_cost(3);
    let toshiba_total_cost = toshiba_laptop.calculate_total_cost(3);
    let dell_total_cost = dell_laptop.calculate_total_cost(3);

    // Calculate the overall total cost
    let overall_total_cost = hp_total_cost + ibm_total_cost + toshiba_total_cost + dell_total_cost;

    // Display the total cost for each brand and the overall total cost
    println!("Total cost for 3 HP laptops: ₦{}", hp_total_cost);
    println!("Total cost for 3 IBM laptops: ₦{}", ibm_total_cost);
    println!("Total cost for 3 Toshiba laptops: ₦{}", toshiba_total_cost);
    println!("Total cost for 3 Dell laptops: ₦{}", dell_total_cost);
    println!("Overall total cost: ₦{}", overall_total_cost);
}

