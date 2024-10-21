use std::io;

fn main() {
    println!("Simple Calculator");

    let mut last_result: Option<f64> = None;

    loop {
        let mut first_num = String::new();
        let mut second_num = String::new();
        let mut operator = String::new();

        // Вибір першого числа
        println!("Enter the first number (or type 'last' to use the last result):");
        io::stdin().read_line(&mut first_num).expect("Failed to read line");

        let first_num: f64 = match first_num.trim() {
            "last" => match last_result {
                Some(res) => res,
                None => {
                    println!("Error: No previous result available.");
                    continue;
                }
            },
            _ => match first_num.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error: Invalid input for the first number.");
                    continue;
                }
            },
        };

        // Вибір оператора
        println!("Enter the operator (+, -, *, /):");
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator: char = match operator.trim().chars().next() {
            Some(op) if "+-*/".contains(op) => op,
            _ => {
                println!("Error: Invalid operator.");
                continue;
            }
        };

        // Вибір другого числа
        println!("Enter the second number:");
        io::stdin().read_line(&mut second_num).expect("Failed to read line");

        let second_num: f64 = match second_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Invalid input for the second number.");
                continue;
            }
        };

        // Виконання обчислень
        let result = match operator {
            '+' => first_num + second_num,
            '-' => first_num - second_num,
            '*' => first_num * second_num,
            '/' => {
                if second_num == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue;
                }
                first_num / second_num
            },
            _ => {
                println!("Error: Unknown operator.");
                continue;
            }
        };

        // Виведення результату і збереження його
        println!("Result: {} {} {} = {}", first_num, operator, second_num, result);
        last_result = Some(result);

        // Запит чи продовжувати
        println!("Do you want to perform another calculation? (y/n):");
        let mut continue_calc = String::new();
        io::stdin().read_line(&mut continue_calc).expect("Failed to read line");

        if continue_calc.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Calculator exited.");
}
