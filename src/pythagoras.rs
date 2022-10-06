use std::io;
pub fn pythtagoras_theorem() {
    println!("Please Enter the First Side");
    let mut side1 = String::new();
    io::stdin()
        .read_line(&mut side1)
        .expect("Failed to read line");
    let side1: f64 = side1.trim().parse().expect("Please Enter a Number");

    println!("Please Enter the Second Side");
    let mut side2 = String::new();
    io::stdin()
        .read_line(&mut side2)
        .expect("Failed to read line");
    let side2: f64 = side2.trim().parse().expect("Please Enter a Number");

    let side3 = (side1.powi(2) + side2.powi(2)).sqrt();

    println!("The Third Side is {}", side3);
}
