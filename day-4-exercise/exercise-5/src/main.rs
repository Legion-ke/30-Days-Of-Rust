fn gcd_manual( mut x: u64,mut y: u64) -> u64 {
    while y != 0 {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x
}

fn main(){
    let num1 = 64;
    let num2 = 56;
    let result = gcd_manual(num1,num2);
    println!("The GCD of {} and {} is {}",num1,num2,result);
}

