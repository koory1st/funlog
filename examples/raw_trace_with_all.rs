use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "trace");
    }
    env_logger::init();
    test(5, 10);
}

#[funlog(trace, all, retVal)]
fn test(a: i32, b: i32) -> i32 {
    println!("Trace example with all params and return value");
    a * b
}