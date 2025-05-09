# funlog

[中文文档](README.zh-CN.md)

A Rust attribute macro for function logging with configurable options.

## Features

- Log function entry and exit with file and line information
- Parameter value logging with type information
- Return value logging with type information
- Support for Result and Option types
- Generic type support with Debug trait
- Configurable log levels (debug, info, warn, error, trace)
- Output length limits for parameters and return values
- Support for nested function calls
- Module path information in logs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
funlog = "0.1.0"
log = "0.4"
```

And initialize a logger in your main function:

```rust
fn main() {
    env_logger::init();
    // Your code here
}
```

## Usage Examples

### Basic Function Logging

```rust
use funlog::funlog;

#[funlog(param, ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(5, 3);
}
```

Output:
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:4 my_app::add(a: 5, b: 3) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:4 my_app::add(a: 5, b: 3)->8 end
```

### Result Type Handling

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    #[funlog(param, ret)]
    fn new(name: String, age: u32) -> Result<Person, String> {
        if age > 150 {
            Err("Age is too high".to_string())
        } else {
            Ok(Person { name, age })
        }
    }
}

fn main() {
    let person = Person::new("Alice".to_string(), 25);
}
```

Output:
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:16 my_app::new(name: "Alice", age: 25) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:16 my_app::new(name: "Alice", age: 25)->Ok(Person { name: "Alice", age: 25 }) end
```

### Option Type Handling

```rust
#[derive(Debug)]
struct Book {
    title: String,
}

impl Book {
    #[funlog(param, ret)]
    fn new(title: String) -> Option<Book> {
        if title.is_empty() {
            None
        } else {
            Some(Book { title })
        }
    }
}

fn main() {
    let book = Book::new("The Great Gatsby".to_string());
}
```

Output:
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:32 my_app::new(title: "The Great Gatsby") begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:32 my_app::new(title: "The Great Gatsby")->Some(Book { title: "The Great Gatsby" }) end
```

### Generic Functions

```rust
#[funlog(param, ret)]
fn process_data<T: std::fmt::Debug>(data: Vec<T>) -> Result<Option<Vec<T>>, String> {
    if data.is_empty() {
        Ok(None)
    } else {
        Ok(Some(data))
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let processed = process_data(numbers);
}
```

Output:
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:42 my_app::process_data(data: [1, 2, 3, 4, 5]) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:42 my_app::process_data(data: [1, 2, 3, 4, 5])->Ok(Some([1, 2, 3, 4, 5])) end
```

### Complex Type Handling

```rust
#[funlog(param, ret)]
fn handle_result(result: Result<Option<String>, i32>) -> Option<Result<String, i32>> {
    match result {
        Ok(Some(s)) => Some(Ok(s)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

fn main() {
    let test_result = Ok(Some("Hello".to_string()));
    let handled = handle_result(test_result);
}
```

Output:
```
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:51 my_app::handle_result(result: Ok(Some("Hello"))) begin
[2024-11-29T13:31:35Z INFO  my_app] src/main.rs:51 my_app::handle_result(result: Ok(Some("Hello")))->Some(Ok("Hello")) end
```

### Log Level Configuration

```rust
#[funlog(param, ret, debug)]
fn debug_function(x: i32) -> i32 {
    x * 2
}

#[funlog(param, ret, warn)]
fn warn_function(x: i32) -> i32 {
    x * 3
}
```

Output:
```
[2024-11-29T13:31:35Z DEBUG my_app] src/main.rs:60 my_app::debug_function(x: 10) begin
[2024-11-29T13:31:35Z DEBUG my_app] src/main.rs:60 my_app::debug_function(x: 10)->20 end
[2024-11-29T13:31:35Z WARN  my_app] src/main.rs:65 my_app::warn_function(x: 10) begin
[2024-11-29T13:31:35Z WARN  my_app] src/main.rs:65 my_app::warn_function(x: 10)->30 end
```

## Notes

1. For generic types and custom structs, the `Debug` trait must be implemented:
```rust
#[derive(Debug)]
struct MyStruct { /* ... */ }
```

2. The macro automatically handles:
   - Result types (Ok/Err)
   - Option types (Some/None)
   - Generic types (with Debug trait)
   - Custom structs (with Debug trait)

3. Log format includes:
   - Timestamp
   - Log level
   - Module path
   - File name and line number
   - Function name
   - Parameter names and values
   - Return value

4. Available log levels:
   - trace
   - debug
   - info (default)
   - warn
   - error

## Test
```
cargo test -- --nocapture --test-threads=1
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
