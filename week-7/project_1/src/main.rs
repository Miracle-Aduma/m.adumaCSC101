
use std::io;

fn read_number(message: &str) -> f64 {
    let mut input = String::new();

    println!("{}", message);
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().parse::<f64>().expect("Please enter a valid number")
}

fn trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

fn cylinder_volume(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("===== THE SHAPE CALCULATOR =====");
    println!("1. Trapezium - Area");
    println!("2. Rhombus - Area");
    println!("3. Parallelogram - Area");
    println!("4. Cube - Surface Area");
    println!("5. Cylinder - Volume");

    let mut choice = String::new();

    println!("Enter your choice (1-5):");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = choice.trim().parse()
        .expect("Please enter a number from 1 to 5");

    match choice {
        1 => {
            let height = read_number("Enter the height:");
            let base1 = read_number("Enter the first base:");
            let base2 = read_number("Enter the second base:");

            let result = trapezium_area(height, base1, base2);
            println!("Area of trapezium = {:.2}", result);
        }

        2 => {
            let diagonal1 = read_number("Enter the first diagonal:");
            let diagonal2 = read_number("Enter the second diagonal:");

            let result = rhombus_area(diagonal1, diagonal2);
            println!("Area of rhombus = {:.2}", result);
        }

        3 => {
            let base = read_number("Enter the base:");
            let altitude = read_number("Enter the altitude:");

            let result = parallelogram_area(base, altitude);
            println!("Area of parallelogram = {:.2}", result);
        }

        4 => {
            let side = read_number("Enter the side length:");

            let result = cube_surface_area(side);
            println!("Surface area of cube = {:.2}", result);
        }

        5 => {
            let radius = read_number("Enter the radius:");
            let height = read_number("Enter the height:");

            let result = cylinder_volume(radius, height);
            println!("Volume of cylinder = {:.2}", result);
        }

        _ => {
            println!("Invalid choice. Please select a number from 1 to 5.");
        }
    }
}