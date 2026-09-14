// Rust program to count nummbers

use std::io;

fn main() {

    println!("Enter lower bound ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let lower_bound:i32 = input1.trim().parse().expect("Failed to read input");

    println!("Enter upper bound ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let  upper_bound:i32 = input2.trim().parse().expect("Not a valid number");

    for x in lower_bound..upper_bound{
        // upper_bound is not inclusive
        println!("Count level is {}", x);
    }

}
