use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input to an integer
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Enter a valid integer.");
            return;
        }
    };

    let square = number * number;
    println!("The square of {} is: {}", number, square);
}