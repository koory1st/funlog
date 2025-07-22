use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "trace");
    }
    env_logger::init();
    
    println!("=== Testing different log levels ===");
    test_trace();
    test_debug();
    test_info();
    test_warn();
    test_error();
    test_print();
}

#[funlog(trace)]
fn test_trace() -> &'static str {
    "trace level"
}

#[funlog(debug)]
fn test_debug() -> &'static str {
    "debug level"
}

#[funlog(info)]
fn test_info() -> &'static str {
    "info level"
}

#[funlog(warn)]
fn test_warn() -> &'static str {
    "warn level"
}

#[funlog(error)]
fn test_error() -> &'static str {
    "error level"
}

#[funlog(print)]
fn test_print() -> &'static str {
    "print level"
}