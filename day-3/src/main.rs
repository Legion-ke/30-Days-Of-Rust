use std::io;

fn main(){
    // println!("Enter number: ");
    // let mut user_input = String::new();
    //
    // io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("Failed to read line");
    //
    // let number: i32 = user_input
    //     .trim()
    //     .parse()
    //     .expect("Invalid input, please enter a valid number");
    // println!("You have entered the number: {}",number);
    //
    // if number % 2 == 0 {
    //     println!("Even");
    // } else if number % 2 == 1 {
    //     println!("Odd number");
    // } else {
    //     println!("Invalid number");
    // }

    // let mut count = 0;
    // loop {
    //     println!("Number: {}", count);
    //     count += 1;
    //     if count == 5 {
    //         break;
    //     }
    // }
    // let week = "friday";
    // match week {
    //     "monday" => println!("Start of week"),
    //     "tuesday" => println!("Second day of the week"),
    //     "wednesday" => println!("Third day of the week"),
    //     "thursday" => println!("Fourth day"),
    //     "friday" => println!("weekend is coming!"),
    //     "saturday" => println!("weekend is here"),
    //     "sunday" => println!("last day of the week"),
    //     _=> println!("Not a day"),
    // }
    println!("Enter Number:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading input");
    let number: i32 = user_input
        .trim()
        .parse()
        .expect("Error converting to interger");
    let result = factorial(number);
    println!("Factorial is: {}",result);

}

fn factorial(mut number: i32) -> i32 {
    let mut fact = 1;

    while number > 1 {
        fact  *= number;
        number -= 1;
    }

    fact
}
