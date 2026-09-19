use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read input");

    println!("Enter your age:");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");

    let experience = experience.trim();
    let age: i32 = age.trim().parse().expect("Please enter a valid age");

    if experience == "yes" && age >= 40 {
        println!("Annual Incentive = N1,560,000");
    } else if experience == "yes" && age >= 30 && age <= 39 {
        println!("Annual Incentive = N1,480,000");
    } else if experience == "yes" && age < 28 {
        println!("Annual Incentive = N1,300,000");
    } else {
        println!("Annual Incentive = N100,000");
    }
}