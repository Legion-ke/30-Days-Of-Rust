fn level(){
    let my_interger: i32 = 42;
    println!("My Interger: {}.",my_interger);
    let my_float: f32 = 4.25;
    println!("My float: {}.",my_float);

    let is_learning_rust: bool = true;
    println!("Is learning rust fun: {}.",is_learning_rust);
    let favourite_letter: char = 'D';
    println!("My favourite letter: {}", favourite_letter);
    let my_score: [i32,5] = [50,60,70,80,90];
    println!("My test scores: {}",my_score);
    let hobby: &str = "Football and basketball";

    println!("I enjoy {}", hobby);
}
