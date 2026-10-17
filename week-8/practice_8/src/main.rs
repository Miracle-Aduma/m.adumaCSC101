fn main() {
    // initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    // Change 3rd and 4th element of a muatble tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8515;

    println!("Changed tuple = {:?}", mountain_heights);
}
