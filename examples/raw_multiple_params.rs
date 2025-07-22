use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    test(1, 2, 3);
}

#[funlog(debug, params(a, c))]
fn test(a: i32, b: i32, c: i32) -> i32 {
    println!("Hello, world! a={}, b={}, c={}", a, b, c);
    a + b + c
}