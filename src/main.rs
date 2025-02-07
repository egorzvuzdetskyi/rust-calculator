pub mod calculator_errors;
pub mod input_operations;
pub mod math;
use crate::calculator_errors::*;
use crate::input_operations::*;
use crate::math::*;

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
