fn multication_table(x: u32){ 
    for i in 1..11{
        let result = x * i;
        println!("{} * {} = {}",x, i,result);
    }

}
fn main() {
    multication_table(5);
}
