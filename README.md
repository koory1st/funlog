# funlog

[简体中文](README.zh-CN.md) | English

A procedural macro for tracing Rust function calls.

## Features

- Automatically trace function entry and exit
- Log function parameters and return values
- Easy integration with the standard Rust logging system
- Minimal performance overhead

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
funlog = "0.1.0"
```

## Usage

```rust
use funlog::trace;

#[trace]
fn example_function(x: i32) -> i32 {
    x + 1
}
```

## Configuration

The macro uses the standard Rust `log` crate. Make sure to initialize a logger (such as `env_logger`) in your application.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
