use std::io;

fn parse_input() -> f64 {
    let mut x: String = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            return f64::NAN;
        }
    };
    return x;
}

fn main() {
    let result: f64;
    println!("Enter the first number: ");
    let x:f64 = parse_input(); 
    println!("Enter the Second number: ");
    let y:f64 = parse_input();

    println!("List of operators");
    println!("1. +");
    println!("2. -");
    println!("3. /");
    println!("4. *");
    println!("Choose the operator: ");

    let op:f64 = parse_input();

    if f64::is_nan(op) {
        println!("Invalid operator");
        return;
    }

    let op:i32 = op as i32;


    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x / y,
        4 => result = x * y,
        _ => result = {
            println!("Invalid operator");
            return;
        }
    }
    println!("Result: {}", result);
}
