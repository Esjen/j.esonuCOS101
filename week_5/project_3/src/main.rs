//Rust program to display a menu and calculate prices
//based on the given order

//Rust code to display a menu for the food items
//available to take order from the customer

fn main() {
    let menu = [("P", "Pounded yam or  Edikaiko Soup"),
                 ("F", "Fried Rice and Chicken"),
                 ("A", "Amala and Ewedu Soup"),
                 ("E", "Eba and Egusi Soup"),
                 ("W", "White rice and Stew"),
                 ];
    //Display menu
    println!("Menu:");
    for item in menu.iter() {
    println!("{}", item.0);
    } 

    let prices:f32 = [3200.0,
                      3000.0,
                      2500.0,
                      2000.0,
                      2500.0]; 
    //Display prices 
    println!("Prices: ");
    for price in prices.iter() {
    println!("{} (in naira)", price.2);
    }
    
    // Get the users order and quantity 
    println!("\nPlease enter the type of food you want to order:");
    let food_type = get_user_input().chars().next().unwrap().to_ascii_uppercase();
    let selected_item = menu.iter().find(|item| item.0 == food_type);

    println!("\nPlease Enter the quantity: ");
    let quantity = get_user_input().parse()::<u32>().unwrap();

    //Calculate the total charges
    let total_charges:f32 = match selected_item {
       Some(item) => price.2 * quantity,
       None => {
       println!("Invalid food type!");
       return;
       }
    }
    
    //Apply discount if total charges > 10000 naira
    let discounted_charges:f32 = if total_charges > 10000.0 {
    let discounted_charges = total_charges * 0.05;
    total_charges - discount as u32
                                 }else {
                                 total_charges
                                 }

fn get_user_input() => String {
 let mut input = String::new();
 std::io::stdin().read_line(&mut input).expect("Failed to read input");
 input.trim().to_string()
}                                                 
    
               

     

    
}