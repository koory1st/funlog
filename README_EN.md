# Funlog

English | [中文](README.md)

A procedural macro for tracing Rust function calls.

## Overview

Funlog is a lightweight Rust procedural macro that automatically logs function call information. It can record function parameters, return values, and supports multiple log levels with flexible configuration options.

## Features

- 🚀 **Zero Runtime Overhead** - Only active in debug builds, completely removed in release builds
- 📝 **Multiple Log Levels** - Supports trace, debug, info, warn, error, and print
- 🎯 **Flexible Parameter Logging** - Choose to log all parameters, specific parameters, or no parameters
- ⏰ **Position Control** - Log at function start, end, or both
- 🔄 **Return Value Logging** - Optionally record function return values
- 🛠️ **Easy to Use** - Simple attribute macro syntax

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
funlog = "0.1.0"

# If using log levels (not print), also add logging libraries
log = "0.4"
env_logger = "0.10"
```

## Quick Start

### Basic Usage

```rust
use funlog::funlog;

#[funlog(debug)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    env_logger::init();
    let result = add(3, 5);
    println!("Result: {}", result);
}
```

### Using print (No Logger Setup Required)

```rust
use funlog::funlog;

#[funlog(print)]
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("World");
}
```

## Configuration Options

### Log Levels

- `print` - Uses `println!` macro (no logger setup required)
- `trace` - Uses `log::trace!`
- `debug` - Uses `log::debug!`
- `info` - Uses `log::info!`
- `warn` - Uses `log::warn!`
- `error` - Uses `log::error!`

### Parameter Logging Options

- `all` - Log all function parameters (default)
- `none` - Log no parameters
- `params(param1, param2, ...)` - Log specific parameters

### Position Control Options

- `onStart` - Log only at function start
- `onEnd` - Log only at function end
- `onStartEnd` - Log at both start and end (default)

### Return Value Logging

- `retVal` - Include return value in logging

## Usage Examples

### Log All Parameters and Return Value

```rust
#[funlog(info, all, retVal)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

### Log Specific Parameters

```rust
#[funlog(debug, params(name, age))]
fn create_user(name: &str, age: u32, email: &str) -> String {
    format!("User: {} ({})", name, age)
}
```

### Log Only at Function End

```rust
#[funlog(warn, onEnd, retVal)]
fn expensive_calculation() -> f64 {
    // Complex calculation
    std::thread::sleep(std::time::Duration::from_millis(100));
    42.0
}
```

### Combine Multiple Options

```rust
#[funlog(info, onStartEnd, params(input), retVal)]
fn process_data(input: &str, config: &Config) -> Result<String, Error> {
    // Processing logic
    Ok(input.to_uppercase())
}
```

## Example Projects

The project includes rich examples demonstrating all feature combinations:

```bash
# Run basic examples
cargo run --example raw_debug

# Run parameter logging examples
cargo run --example raw_all
cargo run --example raw_params

# Run position control examples
cargo run --example raw_position_start
cargo run --example raw_position_end

# Run return value logging examples
cargo run --example raw_return_value

# View all examples
ls examples/
```

For detailed example descriptions, see [examples/README.md](examples/README.md).

## Testing

Run all tests:

```bash
# Run unit tests
cargo test

# Run example test script
./run_tests.sh
```

## How It Works

Funlog is a procedural macro that analyzes functions at compile time and generates corresponding logging code. It only works in debug builds and is completely removed in release builds, ensuring zero runtime overhead.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Issues and Pull Requests are welcome!

## Author

- Levy Gu <32436334@qq.com>

## Changelog

### v0.1.0
- Initial release
- Support for multiple log levels
- Flexible parameter and return value logging
- Position control support
- Complete test coverage and examples