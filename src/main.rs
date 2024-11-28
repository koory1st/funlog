use tlog::tlog;

// Basic function with no parameters
#[tlog]
fn hello() {
    println!("Hello!");
}

// Function with parameters
#[tlog(param)]
fn greet<'a>(name: &'a str) {
    println!("Hello, {}!", name);
}

// Function with return value
#[tlog(ret)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Generic function with type info
#[tlog(param, gener)]
fn process_data<T: std::fmt::Display + std::fmt::Debug>(item: T) -> T {
    println!("Processing: {}", item);
    item
}

// Function with length limits
#[tlog(param="10", ret="10")]
fn repeat_string(text: &str) -> String {
    text.repeat(3)
}

// Function with debug level
#[tlog(debug)]
fn debug_operation() -> i32 {
    println!("Performing debug operation");
    42
}

// Nested function calls
#[tlog(param, ret)]
fn calculate(x: i32, y: i32) -> i32 {
    let sum = add(x, y);
    sum * 2
}

fn main() {
    // Initialize the logger
    env_logger::init();
    
    println!("\n=== Basic Function ===");
    hello();
    
    println!("\n=== Function with Parameters ===");
    greet("Alice");
    
    println!("\n=== Function with Return Value ===");
    let result = add(5, 3);
    println!("Result: {}", result);
    
    println!("\n=== Generic Function ===");
    let processed = process_data("Hello, World!");
    println!("Processed result: {}", processed);
    let num_result = process_data(42);
    println!("Processed number: {}", num_result);
    
    println!("\n=== Function with Length Limits ===");
    let long_text = repeat_string("abcdefghijklmnopqrstuvwxyz");
    println!("Long text result: {}", long_text);
    
    println!("\n=== Debug Level Logging ===");
    let debug_result = debug_operation();
    println!("Debug result: {}", debug_result);
    
    println!("\n=== Nested Function Calls ===");
    let calc_result = calculate(10, 5);
    println!("Calculation result: {}", calc_result);
}
