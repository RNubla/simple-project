use std::io;
mod pythagoras;

fn main() {
    println!("Hello, world!");
    println!("Welcome to my Application");
    println!("Please Enter Your Name");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello {}!", name.trim());

    println!("Please Select an Operation You want to do");
    println!("[1]. Pythagoras Theorem");
    println!("[q]. Quit");

    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    loop {
        match operation.trim() {
            "1" => {
                println!("You Selected Pythagoras Theorem");
                pythagoras::pythtagoras_theorem();
                println!("Would you like to do another calculation? [y/n]");
                let mut continue_calculation = String::new();
                io::stdin()
                    .read_line(&mut continue_calculation)
                    .expect("Expected [y/n]");
                if continue_calculation.trim() == "y" {
                    continue;
                } else if continue_calculation.trim() == "n" {
                    break;
                } else {
                    println!("Invalid Input");
                    continue;
                }
            }
            "q" => {
                println!("Quitting");
                break;
            }
            _ => println!("Invalid Operation"),
        }
    }
}
