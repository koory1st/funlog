use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    
    println!("=== Testing onStart ===");
    test_start(1, 2);
    
    println!("\n=== Testing onEnd ===");
    test_end(3, 4);
    
    println!("\n=== Testing onStartEnd ===");
    test_both(5, 6);
}

#[funlog(debug, onStart, all)]
fn test_start(a: i32, b: i32) -> i32 {
    println!("Inside test_start");
    a + b
}

#[funlog(debug, onEnd, all, retVal)]
fn test_end(a: i32, b: i32) -> i32 {
    println!("Inside test_end");
    a * b
}

#[funlog(debug, onStartEnd, all, retVal)]
fn test_both(a: i32, b: i32) -> i32 {
    println!("Inside test_both");
    a - b
}