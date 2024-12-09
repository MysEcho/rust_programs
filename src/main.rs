fn main() {
    let mut pair : (String, i32) = ("Hello".to_string(),32);
    println!("The value of pair.0 before is {}",pair.0);
    println!("Please enter string!");
    std::io::stdin().read_line(&mut pair.0).expect("Failed to read use input");
    println!("The value of pair.0 after is {}",pair.0);
}

