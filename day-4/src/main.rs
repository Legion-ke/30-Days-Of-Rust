// fn greet_user(name: &str){
//     println!("Hello, {}!",name);
// }

// fn add(a: i32, b: i32) -> i32{
//     a + b
// }

//nested function
// fn outer_function(){
//     fn inner_function(){
//         println!("This the inner function");
//     }
//     inner_function();
// }
//
// function with multiple return values
//
// fn calculate(a: i32, b: i32) -> (i32, i32){
//     (a+b, a*b)
// }
//
// Recursive function

fn factorial(n: u32) -> u32 {
    if n == 0{
        1
    } else {
        n * factorial(n-1)
    }
}

fn main(){
    let result = factorial(5);
    println!("Factorial is {}", result);
}
