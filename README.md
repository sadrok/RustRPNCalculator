# Simple RPN Calculator in Rust

This project is a simple Reverse Polish Notation (RPN) calculator implemented in Rust. It provides a command-line interface for performing basic arithmetic operations such as addition, subtraction, multiplication, division, and modulo operation.

## Features

- Basic arithmetic operations: `+`, `-`, `*`, `/`, `%`
- Stack manipulation commands: `pop`, `clear`, `show`
- Interactive command-line interface
- Error handling for division by zero and insufficient operands

## Usage

To use the calculator, clone the repository and run the program. You will be presented with a command-line interface where you can enter numbers and operators.

### Operators

- `+` : Add the last two numbers on the stack
- `-` : Subtract the last two numbers on the stack
- `*` : Multiply the last two numbers on the stack
- `/` : Divide the last two numbers on the stack
- `%` : Get the remainder of the last two numbers on the stack

### Commands

- `q` : Quit the program
- `p` : Pop the last number from the stack
- `c` : Clear the stack
- `s` : Show the current stack
- `?` : Show the help text

## Example

Here is an example of how to use the calculator:

```
> 5
Number: 5
> 3
Number: 3
> +
Result: 8
> s
Stack: [8]
> q
Final stack: [8]
```

In this example, we first push `5` and `3` onto the stack. We then apply the `+` operator, which pops the last two numbers off the stack, adds them, and pushes the result back onto the stack. We then show the current stack with the `s` command, and finally quit the program with the `q` command.

## Contributing

While this project is not currently accepting contributions, you are more than welcome to fork it and make modifications for your own use.
