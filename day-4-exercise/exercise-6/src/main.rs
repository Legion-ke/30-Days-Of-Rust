fn fibunacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibunacci(n - 1) + fibunacci(n - 2)
    }
}

fn main() {
    let result = fibunacci(10);
    println!("{}", result);
}
