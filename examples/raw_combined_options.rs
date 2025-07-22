use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "info");
    }
    env_logger::init();
    test(10, 20);
}

#[funlog(info, onStartEnd, params(a), retVal)]
fn test(a: i32, b: i32) -> i32 {
    println!("Computing: {} + {}", a, b);
    a + b
}