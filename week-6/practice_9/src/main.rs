fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of A:{}", a);
    println!("Value of B:{}", b);

    let res = a > b;
    println!("A is greater than B: {}", res);

    let res = a < b;
    println!("A is lesser than B: {}", res);

    let res = a >= b;
    println!("A is greater than or equal to B: {}", res);

    let res = a <= b;
    println!("A is lesser than  or equal to B: {}", res);

    let res = a == b;
    println!("A is equal to B: {}", res);

    let res = a != b;
    println!("A is not equal to B: {}", res);
}
