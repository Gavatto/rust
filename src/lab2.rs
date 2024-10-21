use std::io;

fn main() {
    println!("Simple Calculator");

    let mut first_num = String::new();
    let mut second_num = String::new();
    let mut operator = String::new();

    // Read the first number
    println!("Enter the first number:");
    io::stdin().read_line(&mut first_num).expect("Failed to read line");
    let first_num: f64 = first_num.trim().parse().expect("Please type a number");

    // Read the operator
    println!("Enter the operator (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("Failed to read line");
    let operator: char = operator.trim().chars().next().unwrap();

    // Read the second number
    println!("Enter the second number:");
    io::stdin().read_line(&mut second_num).expect("Failed to read line");
    let second_num: f64 = second_num.trim().parse().expect("Please type a number");

    // Perform the calculation
    let result = match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' => first_num * second_num,
        '/' => {
            if second_num == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            first_num / second_num
        },
        _ => {
            println!("Error: Invalid operator.");
            return;
        }
    };

    // Output the result
    println!("Result: {} {} {} = {}", first_num, operator, second_num, result);
}
