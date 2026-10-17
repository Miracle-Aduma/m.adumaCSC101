
fn main() {
    // Store each developer's name and years of experience
    let developers: Vec<(&str, u32)> = vec![
        ("Alice", 3),
        ("Brian", 7),
        ("Chika", 5),
        ("David", 10),
        ("Esther", 6),
    ];

    // Assume the first developer has the highest experience
    let mut highest = developers[0];

    // Check every developer in the vector
    for developer in &developers {
        if developer.1 > highest.1 {
            highest = *developer;
        }
    }

    // Display the developer with the highest experience
    println!(
        "The developer with the highest experience is {} with {} years.",
        highest.0, highest.1
    );
}
