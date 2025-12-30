fn average_array(arr: &[i32]) -> i32 {
    let sum: i32 = arr.iter().sum();
    sum / arr.len() as i32
}

fn main() {
    let number: [i32; 5] = [5, 10, 11, 200, 5];

    let result = average_array(&number);

    println!("{:?}", result);
}
