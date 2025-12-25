fn main(){
    let my_age = 22; //immutable variable

    println!("my age: {}.",my_age);
    
    let mut my_height: i32 = 180; //mutableprintln
    println!("First Height: {}.",my_height);

    my_height = 175;
    println!("Second Height: {}.",my_height);
    let my_name: &str = "Given Omondi";
    println!("My name is {}.",my_name);
    let is_student: bool= false;
    println!("Am i student: {}.",is_student);
    let birth_year = 2025 - my_age;

    println!("Birth year: {}.",birth_year);
}
