use std::io;

fn main() {
    // Display the menu
    println!("================ RESTAURANT MENU ================");
    println!("P - Poundo Yam / Edinkaiko Soup     N3,200");
    println!("F - Fried Rice & Chicken            N3,000");
    println!("A - Amala & Ewedu Soup              N2,500");
    println!("E - Eba & Egusi Soup                N2,000");
    println!("W - White Rice & Stew               N2,500");
    println!("==================================================");

    // Get food type from customer
    println!("Enter food type (P, F, A, E or W):");

    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Get quantity from customer
    println!("Enter quantity:");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");

    let quantity: i32 = quantity
        .trim()
        .parse()
        .expect("Please enter a valid quantity");

    // Determine the price using the food letter
    let price: i32;

    match food_type.as_str() {
        "P" => {
            price = 3200;
        }
        "F" => {
            price = 3000;
        }
        "A" => {
            price = 2500;
        }
        "E" => {
            price = 2000;
        }
        "W" => {
            price = 2500;
        }
        _ => {
            println!("Invalid food type!");
            return;
        }
    }

    // Calculate total charge
    let total = price * quantity;

    println!("Total before discount: N{}", total);

    // Give 5% discount if total is greater than N10,000
    let mut discount = 0;

    if total > 10000 {
        discount = total * 5 / 100;
    }

    // Calculate final amount
    let amount_to_pay = total - discount;

    println!("Discount: N{}", discount);
    println!("Amount to pay: N{}", amount_to_pay);
}
