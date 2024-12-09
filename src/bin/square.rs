use std::io;

fn calculate_square(number: i32) -> i32 {
    number * number
}

// get user input and handle parsing
fn get_user_input() -> Option<i32> {
    println!("Enter an integer:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid input! Please enter a valid integer.");
            None
        }
    }
}

fn main() {
    match get_user_input() {
        Some(number) => {
            let square = calculate_square(number);
            println!("The square of {} is: {}", number, square);
        },
        None => {
            println!("Exiting the program due to invalid input.");
        }
    }
}