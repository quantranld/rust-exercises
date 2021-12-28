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
        if let Some(x) = match input {
            Value(val) => Some(*val),

            Add => apply_calculation(&mut stack, |top, after_top| after_top + top),

            Subtract => apply_calculation(&mut stack, |top, after_top| after_top - top),

            Multiply => apply_calculation(&mut stack, |top, after_top| after_top * top),

            Divide => apply_calculation(&mut stack, |top, after_top| after_top / top),
        } {
            stack.push(x);
        } else {
            return None;
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn apply_calculation<TFn>(stack: &mut Vec<i32>, executor: TFn) -> Option<i32>
where
    TFn: FnOnce(i32, i32) -> i32,
{
    if stack.len() < 2 {
        return None;
    }

    Some(executor(stack.pop()?, stack.pop()?))
}
