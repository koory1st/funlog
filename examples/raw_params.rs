use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    test(1, 2);
}

#[funlog(params(a))]
fn test(a: i32, b: i32) -> i32 {
    println!("Hello, world! a={}, b={}", a, b);
    a + b
}
