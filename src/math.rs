use crate::calculator_errors::*;

pub fn do_math_operation(n1: f32, n2: f32, oper: &str) -> Result<f32, CalculatorError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_operation() {
        assert_eq!(
            do_math_operation(1.0, 2.0, &String::from("+")).unwrap(),
            3.0
        )
    }

    #[test]
    fn test_minus_operation() {
        assert_eq!(
            do_math_operation(1.0, 2.0, &String::from("-")).unwrap(),
            -1.0
        )
    }

    #[test]
    fn test_divide_operation() {
        assert_eq!(
            do_math_operation(4.0, 2.0, &String::from("/")).unwrap(),
            2.0
        )
    }

    #[test]
    fn test_multiplication_operation() {
        assert_eq!(
            do_math_operation(4.0, 2.0, &String::from("*")).unwrap(),
            8.0
        )
    }

    #[test]
    fn test_divide_by_zero() {
        let result = do_math_operation(1.0, 0.0, &String::from("/"))
            .unwrap_err()
            .to_string();
        assert_eq!(result, String::from("Division by zero is not allowed"))
    }

    #[test]
    fn test_unkown_operation() {
        let operation = String::from("invalid-operation");
        let result = do_math_operation(1.0, 0.0, &operation)
            .unwrap_err()
            .to_string();

        println!("{result}");
        assert_eq!(result, String::from("Invalid operation: invalid-operation"))
    }
}
