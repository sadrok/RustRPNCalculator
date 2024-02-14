//! SIMPLE RPN CALCULATOR

use std::io::{stdout, Write};

// Define base numeric value as f32
type Number = f32;

// STRUCTS/ENUMS

/// Various known errors that can occur when applying an operator
#[derive(Debug)]
enum OperatorError {
    DivideByZero,
    ModuloByZero,
    NotEnoughOperands,
}

/// Various application commands
enum Commands {
    Quit,
    Pop,
    Show,
    Clear,
    Help,
}

/// An operator that acts upon the operand stack
trait Operator {
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError>;
}

// OPERATORS
/// Add two numbers
struct Adder {}
impl Operator for Adder {
    /// Pop two numbers off the stack, add them, and push the result back onto the stack.
    /// It raises an error if there are not enough numbers on the stack.
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError> {
        if operand_stack.len() < 2 {
            return Err(OperatorError::NotEnoughOperands);
        }

        let a = operand_stack.pop().unwrap();
        let b = operand_stack.pop().unwrap();

        let answer = a + b;
        operand_stack.push(answer);
        Ok(answer)
    }
}

/// Subtract two numbers, or negate the individual number on the stack
struct Subtractor {}
impl Operator for Subtractor {
    /// Pop two numbers off the stack, subtract them, and push the result back onto the stack.
    /// The top-most number on the stack is subtracted from the second number on the stack.
    /// If there is only one number on the stack, negate it.
    /// It raises and error if the stack is empty.
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError> {
        match operand_stack.len() {
            0 => Err(OperatorError::NotEnoughOperands),
            1 => {
                let b = operand_stack
                    .pop()
                    .ok_or(OperatorError::NotEnoughOperands)?;
                let answer = -b;
                operand_stack.push(answer);
                Ok(answer)
            }
            _ => {
                let b = operand_stack.pop().unwrap();
                let a = operand_stack.pop().unwrap();
                let answer = a - b;
                operand_stack.push(answer);
                Ok(answer)
            }
        }
    }
}

/// Multiply two numbers
struct Multiplier {}
impl Operator for Multiplier {
    /// Pop two numbers off the stack, multiply them, and push the result back onto the stack.
    /// It raises an error if there are not enough numbers on the stack.
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError> {
        if operand_stack.len() < 2 {
            return Err(OperatorError::NotEnoughOperands);
        }

        let a = operand_stack.pop().unwrap();
        let b = operand_stack.pop().unwrap();
        let answer = a * b;
        operand_stack.push(answer);
        Ok(answer)
    }
}

/// Divide two numbers
struct Divider {}
impl Operator for Divider {
    /// Pop two numbers off the stack, divide them, and push the result back onto the stack.
    /// The top-most number on the stack is divided by the second number on the stack.
    /// It raises an error if there are not enough numbers on the stack.
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError> {
        if operand_stack.len() < 2 {
            return Err(OperatorError::NotEnoughOperands);
        }

        let b = operand_stack.pop().unwrap();
        let a = operand_stack.pop().unwrap();
        if b == Number::from(0u8) {
            operand_stack.push(a);
            operand_stack.push(b);
            Err(OperatorError::DivideByZero)
        } else {
            let answer = a / b;
            operand_stack.push(answer);
            Ok(answer)
        }
    }
}

/// Get the remainder of two numbers
struct Modulator {}
impl Operator for Modulator {
    /// Pop two numbers off the stack, get the remainder, and push the result back onto the stack.
    /// The top-most number on the stack is divided by the second number on the stack.
    /// It raises an error if there are not enough numbers on the stack.
    fn apply(&self, operand_stack: &mut Vec<Number>) -> Result<Number, OperatorError> {
        if operand_stack.len() < 2 {
            return Err(OperatorError::NotEnoughOperands);
        }

        let b = operand_stack.pop().unwrap();
        let a = operand_stack.pop().unwrap();
        if b == Number::from(0u8) {
            operand_stack.push(a);
            operand_stack.push(b);
            Err(OperatorError::ModuloByZero)
        } else {
            let answer = a % b;
            operand_stack.push(answer);
            Ok(answer)
        }
    }
}

// HELPER FUNCTIONS

/// Read a line of input from stdin, will include the newline character.
/// It first outputs a prompt.
/// If, for some reason, it fails to read a line, it will panic.
fn read_input() -> String {
    let mut input = String::new();
    print!("> ");
    stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// Try and get an operator from a string, returns None if it doesn't match. It ignores whitespace.
fn try_get_operator(str: &str) -> Option<Box<dyn Operator>> {
    match str.trim() {
        "+" => Some(Box::new(Adder {})),
        "-" => Some(Box::new(Subtractor {})),
        "*" => Some(Box::new(Multiplier {})),
        "/" => Some(Box::new(Divider {})),
        "%" => Some(Box::new(Modulator {})),
        _ => None,
    }
}

/// Try and get an application command from a string, returns None if it doesn't match. It ignores whitespace.
fn try_get_command(str: &str) -> Option<Commands> {
    match str.trim() {
        "q" => Some(Commands::Quit),
        "p" => Some(Commands::Pop),
        "c" => Some(Commands::Clear),
        "s" => Some(Commands::Show),
        "?" => Some(Commands::Help),
        _ => None,
    }
}

/// Try and get a number from a string, returns None if the string does not contain a number. It ignores whitespace.
fn try_get_number(str: &str) -> Option<Number> {
    str.trim().parse::<Number>().ok()
}

/// Print out the help text
fn print_help() {
    println!("Valid operators: +, -, *, /, %");
    println!("Valid commands: (q)uit, (p)op, (s)how, (c)lear, ?");
}
// MAIN FUNCTION
fn main() {
    let operand_stack: &mut Vec<Number> = &mut Vec::new();

    print_help();

    loop {
        let line = read_input();
        if line.trim().is_empty() {
            continue;
        }

        // Check if we have an app-level operation
        if let Some(operation) = try_get_command(&line) {
            match operation {
                Commands::Help => {
                    print_help();
                }
                Commands::Quit => break,
                Commands::Pop => match operand_stack.pop() {
                    Some(number) => println!("Popped: {}", number),
                    None => println!("Stack is empty"),
                },
                Commands::Clear => {
                    println!("Clearing stack: {:?}", operand_stack);
                    operand_stack.clear();
                }
                Commands::Show => {
                    println!("Stack: {:?}", operand_stack);
                }
            }
        }
        // Check if its a mathematical operator
        else if let Some(operator) = try_get_operator(&line) {
            match operator.apply(operand_stack) {
                Ok(number) => {
                    println!("Result: {}", number);
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                }
            }
        }
        // Check if its a number
        else if let Some(number) = try_get_number(&line) {
            println!("Number: {}", number);
            operand_stack.push(number);
        }
        // Invalid input...
        else {
            println!("Invalid input");
        }
    }

    println!("Final stack: {:?}", operand_stack);
}
