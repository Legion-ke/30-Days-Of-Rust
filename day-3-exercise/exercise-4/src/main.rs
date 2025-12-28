use std::io;

fn main() {
    let result: f64;
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();

    println!("Enter the first number: ");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let x: f64 = x.trim().parse().expect("Please type a number!");

    println!("Enter the Second number: ");
    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
    let y: f64 = y.trim().parse().expect("Please type a number!");


    println!("List of operators");
    println!("1. +");
    println!("2. -");
    println!("3. /");
    println!("4. *");
    println!("Choose the operator: ");

    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read line");

    let op: i32 = op.trim().parse().expect("Please type a number!");

    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x / y,
        4 => result = x * y,
        _ => result = 0.0,
    }

    println!("Result: {}", result);
}
