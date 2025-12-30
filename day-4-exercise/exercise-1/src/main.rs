fn length_string(n: &str) -> i32 {
    n.len() as i32
}

fn main(){
    let word = length_string("almony");

    println!("length of word: {}.", word);
}
