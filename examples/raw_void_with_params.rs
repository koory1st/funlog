use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    test(42, "hello");
}

#[funlog(debug, all)]
fn test(num: i32, msg: &str) {
    println!("Processing: {num} - {msg}");
}
