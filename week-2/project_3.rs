fn main() {
    // Principal / initial value of the TV
    let p: f64 = 210_000.0;

    // Rate of depreciation (5% per annum)
    let r: f64 = 5.0;

    // Time period in years
    let n: f64 = 3.0;

    // Formula: A = P * [1 - (R / 100)]^n
    // In Rust, exponents for f64 are calculated using .powf()
    let a: f64 = p * (1.0 - (r / 100.0)).powf(n);

    // Output the result
    println!("The value of the TV after 3 years is: {:.2}", a);
}