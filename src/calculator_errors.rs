use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidOperation(String),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero is not allowed"),
            CalculatorError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl Error for CalculatorError {}
