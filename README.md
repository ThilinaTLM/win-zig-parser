# WinZig AST Generator

Welcome to the WinZig AST Generator, a tool written in Rust that generates Abstract Syntax Trees (ASTs) for the WinZig programming language. This project provides an easy-to-use interface for analyzing your WinZig code, offering you insights into its structure and aiding in debugging.

This tool leverages Rust's efficient and safe systems programming language capabilities, making the AST generation process reliable and fast.

## Requirements

- Rust (recommend 1.67.1 or higher)
- Cargo (recommend 1.67.1 or higher)

[Install Rust & Cargo](https://www.rust-lang.org/tools/install)

Make sure you have these installed on your machine before proceeding.

## Building From Source

1. Clone this repository to your local machine.
    ```bash
    git clone https://github.com/ThilinaTLM/win-zig-parser.git
    cd win-zig-parser
    ```

2. Build the project using Cargo.
    ```bash
    cargo build --release
    ```

3. Run the executable with sample WinZig source file.
    ```bash
    ./target/release/win-zig winzig_test/winzig_01
    ```

## Tests

Run the following command to run the test suite.

```bash
cargo test
```

This will test all the source files provided in `winzig_test` directory.

## Usage

1. Generate AST
    ```bash
    ./target/release/win-zig <path-to-winzig-file>
    ```

2. Generate AST with verbose output
    ```bash
    ./target/release/win-zig <path-to-winzig-file> --verbose
    ```

3. Get lexical tokens
    ```bash
   ./target/release/win-zig <path-to-winzig-file> --tokens
    ```

