fn conversion(x: f64) -> f64 {
    let c = (x * 1.8) + 32.0;
    c
}

fn main(){
    let con = conversion(50.0);
    println!("{}", con);
}
