mod calculator_errors;
mod input_operations;
use crate::calculator_errors::*;
use crate::input_operations::*;

fn main() {
    let number1 = read_input_number(String::from("Write your first number"));
    let operation = read_input(String::from(
        "What operation you want to do? ex: +, -, /, *",
    ));
    let number2 = read_input_number(String::from("Write your second number"));

    let math_operation_result = do_math_operation(number1, number2, &operation);

    match math_operation_result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => match e {
            CalculatorError::DivisionByZero => println!("Cannot divide by zero!"),
            CalculatorError::InvalidOperation(op) => println!("Invalid operation: {}", op),
        },
    }
}

fn do_math_operation(n1: f32, n2: f32, oper: &str) -> Result<f32, CalculatorError> {
    match oper {
        "+" => Ok(n1 + n2),
        "-" => Ok(n1 - n2),
        "/" => {
            if n2 == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(n1 / n2)
            }
        }
        "*" => Ok(n1 * n2),
        _ => Err(CalculatorError::InvalidOperation(String::from(oper))),
    }
}
