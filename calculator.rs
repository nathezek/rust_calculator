use std::io;
use std::io::Write;

fn main() {
    // CLI Calculator //
    println!("CLI Calculator v.1.0 🧮");

    print!("Enter first number: ");
    io::stdout().flush().expect("Flushing failed");

    // first_number //
    let mut first_number = String::new();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read_line!");

    let first_number: f64 = first_number.trim().parse().expect("Failed to type number!");

    // println!("You entered: {}", first_number);

    // second_number //
    print!("Enter second number :");
    io::stdout().flush().expect("Flushing failed");
    let mut second_number = String::new();

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read_line");

    let second_number: f64 = second_number
        .trim()
        .parse()
        .expect("Failed to type number!");

    // println!("You entered: {}", second_number);

    // choose an operation //
    println!("Choose an operation ['add or +' 'subtract or -' 'multiply or *' 'divide or /'] :");
    let mut operator_input = String::new();
    io::stdin()
        .read_line(&mut operator_input)
        .expect("Failed to read_line!");

    let operation_choice = Operation::from_str(&operator_input);

    // show results
    let results = operator(
        operation_choice.expect("invalid!"),
        first_number,
        second_number,
    );
    println!("Result is: {}", results);
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(operator_input: &str) -> Option<Self> {
        match operator_input.trim().to_lowercase().as_str() {
            "add" | "+" => Some(Operation::Add),
            "subtract" | "-" => Some(Operation::Subtract),
            "multiply" | "*" => Some(Operation::Multiply),
            "divide" | "/" => Some(Operation::Divide),
            _ => None, // Returns None if the input doesn't match
        }
    }
}

fn operator(op: Operation, first_number: f64, second_number: f64) -> f64 {
    match op {
        Operation::Add => first_number + second_number,
        Operation::Subtract => first_number - second_number,
        Operation::Multiply => first_number * second_number,
        Operation::Divide => first_number / second_number,
    }
}

