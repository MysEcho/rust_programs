use std::io;

// Function to square the input number
fn squares(number: i32) -> i32 {
    number * number
}

// Function to add 5 to the input number
fn add(number: i32) -> i32 {
    number + 5
}

// Function to get user input for the number
fn get_number() -> Option<i32> {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid number input!");
            None
        }
    }
}

// Function to get user operation choice
fn get_operation() -> Option<String> {
    println!("Enter operation (add or square):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let operation = input.trim().to_lowercase();

    if operation == "add" || operation == "square" {
        Some(operation)
    } else {
        println!("Invalid operation! Choose 'add' or 'square'.");
        None
    }
}

fn main() {
    let number = match get_number() {
        Some(num) => num,
        None => return,
    };

    let operation = match get_operation() {
        Some(op) => op,
        None => return,
    };

    let result = match operation.as_str() {
        "square" => squares(number),
        "add" => add(number),
        _ => {
            println!("Unexpected error!");
            return;
        }
    };

    println!("Result of {} on {} is: {}", operation, number, result);
}