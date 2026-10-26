
struct Laptop {
    brand: String,
    quantity: u32,
    price: u64,
}

impl Laptop {
    // Method to calculate the cost of buying laptops
    fn purchase_cost(&self, number: u32) -> u64 {
        self.price * number as u64
    }
}

fn main() {
    // Store laptop details in a vector
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            quantity: 10,
            price: 650_000,
        },
        Laptop {
            brand: String::from("IBM"),
            quantity: 6,
            price: 755_000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            quantity: 10,
            price: 550_000,
        },
        Laptop {
            brand: String::from("Dell"),
            quantity: 4,
            price: 850_000,
        },
    ];

    let mut total_cost: u64 = 0;

    // Calculate the cost of 3 laptops from each brand
    for laptop in &laptops {
        let cost = laptop.purchase_cost(3);

        println!("Brand: {}", laptop.brand);
        println!("Quantity available: {}", laptop.quantity);
        println!("Price per laptop: N{}", laptop.price);
        println!("Cost of 3 laptops: N{}", cost);

        total_cost += cost;
    }

    println!("Total cost of purchasing 3 laptops from each brand: N{}", total_cost);
}
