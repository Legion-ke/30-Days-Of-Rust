fn max_num(x: i32, y:i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let number = max_num(50,100);

    println!("The maximum number is {}", number);
}
