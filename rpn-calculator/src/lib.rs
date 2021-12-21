#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            Value(val) => stack.push(*val),
            _ if stack.len() < 2 => return None,
            _ => {
                let top = stack.pop().unwrap();
                let after_top = stack.pop().unwrap();
                match calculate(input, after_top, top) {
                    Some(val) => stack.push(val),
                    None => return None,
                }
            }
        }
    }

    if stack.len() == 1 { stack.pop() } else { None }
}

fn calculate(operator: &CalculatorInput, a: i32, b: i32) -> Option<i32> {
    use CalculatorInput::*;
    let result = match operator {
        Add => a + b,
        Subtract => a - b,
        Multiply => a * b,
        Divide => a / b,
        Value(_) => return None,
    };

    Some(result)
}