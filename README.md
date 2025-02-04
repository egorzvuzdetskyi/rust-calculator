# Rust Calculator

A simple command-line calculator written in Rust that supports basic arithmetic operations.

## Features

- Basic arithmetic operations (+, -, *, /)
- Input validation
- Custom error handling for division by zero
- Docker support (work in progress)

## Prerequisites

- Rust (1.84.1)
- Cargo
- Docker (optional)

## Installation

Clone the repository

Build the project:
```bash
cargo build --release
```

## Usage

### Running Locally

Run the calculator using cargo:
```bash
cargo run
```

The program will prompt you for:
1. First number
2. Operation (+, -, *, /)
3. Second number

Example interaction:
```
Write your first number
1
What operation you want to do? ex: +, -, /, *
+
Write your second number
2
Result: 3
```

### Error Handling

The calculator handles various error cases:
- Division by zero
- Invalid operations
- Invalid number inputs

### Docker Support (Work in Progress)

Build the Docker image:
```bash
docker build -t rust-calculator .
```

Run the container:
```bash
docker run -i rust-calculator
```

Note: Docker input handling is currently being improved. Updates will be available in future releases.

## Project Structure

```
rust-calculator/
├── src/
│   ├── main.rs
│   ├── calculator_errors.rs
│   └── input_operations.rs
├── Cargo.toml
├── Cargo.lock
├── Dockerfile
└── README.md
```

## Testing

Unit tests will be added in upcoming releases. The test suite will cover:
- Basic arithmetic operations
- Error handling
- Edge cases
- Input validation

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Todo

- [ ] Complete Docker input handling
- [ ] Add comprehensive test suite
- [ ] Add support for more complex operations

## Authors

- Your Name (@egorzvuzdetskyi)
