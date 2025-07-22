# Funlog Examples

This directory contains comprehensive examples demonstrating all features of the `funlog` procedural macro.

## Basic Usage Examples

### Log Levels
- `raw_print.rs` - Using `print` (no env_logger needed)
- `raw_trace.rs` - Using `trace` level logging
- `raw_debug.rs` - Using `debug` level logging  
- `raw_info.rs` - Using `info` level logging
- `raw_warn.rs` - Using `warn` level logging
- `raw_error.rs` - Using `error` level logging
- `raw_log_levels_comparison.rs` - Comparing all log levels in one example

### Parameter Logging
- `raw_all.rs` - Log all function parameters with `all`
- `raw_none.rs` - Log no parameters with `none`
- `raw_params.rs` - Log specific parameters with `params(param_name)`
- `raw_multiple_params.rs` - Log multiple specific parameters with `params(a, c)`

### Position Control
- `raw_position_start.rs` - Log only at function start with `onStart`
- `raw_position_end.rs` - Log only at function end with `onEnd`
- `raw_position_start_params.rs` - Log at start with parameters
- `raw_position_end_params.rs` - Log at end with parameters
- `raw_position_start_and_end_params.rs` - Log at both start and end with parameters
- `raw_position_start_and_end_return.rs` - Log at both start and end with return value
- `raw_position_combinations.rs` - Compare all position options

### Return Value Logging
- `raw_return_value.rs` - Log return values with `retVal`
- `raw_position_end_return.rs` - Log return value at function end
- `raw_print_with_return.rs` - Print-based logging with return value
- `raw_trace_with_all.rs` - Trace logging with all params and return value

### Function Types
- `raw_void_function.rs` - Functions that return `()` (void)
- `raw_void_with_params.rs` - Void functions with parameters
- `raw_position_end_void.rs` - Void function with end position logging

### Advanced Examples
- `raw_combined_options.rs` - Multiple options combined: `info, onStartEnd, params(a), retVal`
- `raw_print_with_params.rs` - Print logging with parameters
- `raw_complex_types.rs` - Working with structs and complex types
- `raw_error_handling.rs` - Logging functions that return `Result<T, E>`

## Running Examples

To run any example:

```bash
cargo run --example raw_debug
cargo run --example raw_all
# etc.
```

## Configuration Options

### Log Levels
- `print` - Uses `println!` macro (no logger setup required)
- `trace` - Uses `log::trace!`
- `debug` - Uses `log::debug!`
- `info` - Uses `log::info!`
- `warn` - Uses `log::warn!`
- `error` - Uses `log::error!`

### Parameter Options
- `all` - Log all function parameters
- `none` - Log no parameters
- `params(param1, param2, ...)` - Log specific parameters

### Position Options
- `onStart` - Log only when function starts
- `onEnd` - Log only when function ends
- `onStartEnd` - Log at both start and end (default)

### Return Value
- `retVal` - Include return value in logging

### Combining Options
You can combine multiple options:
```rust
#[funlog(info, onStartEnd, params(a, b), retVal)]
fn my_function(a: i32, b: i32, c: i32) -> i32 {
    // function body
}
```

## Note
The macro only works in debug builds. In release builds, it returns the original function unchanged.