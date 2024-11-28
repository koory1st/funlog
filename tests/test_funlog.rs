use funlog::funlog;

// Basic function with no parameters
#[funlog]
fn hello() {
    println!("Hello!");
}

// Function with parameters
#[funlog(param)]
fn greet<'a>(name: &'a str) {
    println!("Hello, {}!", name);
}

// Function with return value
#[funlog(ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Generic function
#[funlog(param, gener)]
fn print_item<T: std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("{}", item);
}

// Function with length limits
#[funlog(param="10", ret="20")]
fn process_data(data: String) -> String {
    data.repeat(2)
}

// Function with debug log level
#[funlog(debug)]
fn debug_function() {
    println!("Debug function called");
}

// Nested function calls
#[funlog(param, ret)]
fn add1(a: i32, b: i32) -> i32 {
    a + b
}

#[funlog(param, ret)]
fn add2(a: i32, b: i32) -> i32 {
    add1(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_basic_logging() {
        init();
        hello();
    }

    #[test]
    fn test_parameter_logging() {
        init();
        greet("Alice");
    }

    #[test]
    fn test_return_value_logging() {
        init();
        assert_eq!(add(5, 3), 8);
    }

    #[test]
    fn test_generic_logging() {
        init();
        print_item(42);
        print_item("Hello");
    }

    #[test]
    fn test_length_limits() {
        init();
        process_data("This is a very long string that should be truncated".to_string());
    }

    #[test]
    fn test_nested_calls() {
        init();
        let result = add2(5, 3);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_debug_level() {
        init();
        debug_function();
    }
}
