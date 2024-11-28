# funlog

A Rust attribute macro for function logging with configurable options.

## Features

- Log function entry and exit
- Parameter value logging
- Return value logging
- Generic type support
- Configurable log levels
- Output length limits
- Support for nested function calls

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
funlog = "0.1.0"
```

## Usage

### Basic Logging

```rust
use funlog::funlog;

#[funlog]
fn hello() {
    println!("Hello!");
}
```

### Parameter Logging

```rust
#[funlog(param)]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

### Return Value Logging

```rust
#[funlog(ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Generic Function Logging

```rust
#[funlog(param, gener)]
fn print_item<T: std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("{}", item);
}
```

### Length Limits

Limit the length of logged parameters and return values:

```rust
#[funlog(param="10", ret="20")]
fn process_data(data: String) -> String {
    data.repeat(2)
}
```

### Log Levels

Specify different log levels (debug, info, warn, error, trace):

```rust
#[funlog(debug)]
fn debug_function() {
    println!("Debug function called");
}
```

### Nested Function Calls

Track function call chains:

```rust
#[funlog(param, ret)]
fn add1(a: i32, b: i32) -> i32 {
    a + b
}

#[funlog(param, ret)]
fn add2(a: i32, b: i32) -> i32 {
    add1(a, b)
}
```

## Configuration Options

- `param` - Enable parameter logging
- `ret` - Enable return value logging
- `gener` - Enable generic type information logging
- `param="n"` - Limit parameter log length to n characters
- `ret="n"` - Limit return value log length to n characters
- Log levels:
  - `debug`
  - `info` (default)
  - `warn`
  - `error`
  - `trace`

## License

MIT License
