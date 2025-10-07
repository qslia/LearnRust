# Code Explanation: main.rs

## Overview
This Rust program uses the `ferris_says` crate to display a message in ASCII art format, featuring Ferris the Crab (Rust's mascot).

## Import Statements

```rust
use ferris_says::say;
use std::io::{stdout, BufWriter};
```

- **`ferris_says::say`**: Imports the `say` function from the `ferris_says` crate, which formats and displays text in a speech bubble with ASCII art
- **`std::io::{stdout, BufWriter}`**: Imports standard library I/O components:
  - `stdout`: Provides access to the standard output stream
  - `BufWriter`: A buffered writer that improves performance by reducing the number of system calls

## Main Function Breakdown

```rust
fn main() {
```
The entry point of the program.

### Step 1: Get Standard Output Handle
```rust
let stdout = stdout();
```
Creates a handle to the standard output stream (typically the terminal/console).

### Step 2: Create Message
```rust
let message = String::from("Hello fellow Rustaceans!");
```
Creates a `String` from a string literal. "Rustaceans" is the term for Rust programmers.

### Step 3: Calculate Message Width
```rust
let width = message.chars().count();
```
- `message.chars()`: Creates an iterator over the Unicode characters in the string
- `.count()`: Counts the number of characters (important for proper formatting)
- This ensures the ASCII art box is sized correctly for the message

### Step 4: Create Buffered Writer
```rust
let mut writer = BufWriter::new(stdout.lock());
```
- `stdout.lock()`: Locks the standard output for exclusive access, preventing interleaved output from multiple threads
- `BufWriter::new()`: Wraps the locked stdout in a buffered writer for efficient I/O
- `mut`: Makes the writer mutable, as writing requires modifying the writer's internal state

### Step 5: Display the Message
```rust
say(&message, width, &mut writer).unwrap();
```
- `say()`: The function from `ferris_says` that generates the ASCII art
  - `&message`: A reference to the message string
  - `width`: The width for formatting
  - `&mut writer`: A mutable reference to the buffered writer
- `.unwrap()`: Handles the `Result` type returned by `say()`. If an error occurs, the program will panic with an error message. In production code, more robust error handling would be preferred.

## Program Output

When run, this program displays something like:
```
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

## Key Rust Concepts Demonstrated

1. **Ownership and Borrowing**: Uses references (`&message`, `&mut writer`) to pass data without transferring ownership
2. **Error Handling**: Uses `Result` type and `.unwrap()` for error handling
3. **String Types**: Distinguishes between `&str` (string slice) and `String` (owned string)
4. **Standard Library**: Leverages `std::io` for efficient I/O operations
5. **External Crates**: Demonstrates using third-party crates from crates.io
6. **Iterators**: Uses `.chars()` iterator for character counting

