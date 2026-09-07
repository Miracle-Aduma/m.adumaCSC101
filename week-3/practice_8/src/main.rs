// Immutability by default
fn main() {
    let fees = 25_000;
    println!("Fees is {}", fees);

    // Because fees have been asigned a value before,
    //Change the varaible name to something different
    let fees_changed = 35_000;
    println!("Fees changed is {}", fees_changed);
}
