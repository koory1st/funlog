use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    test();
}

#[funlog(debug)]
fn test() -> i32 {
    println!("Hello, world!");
    0
}
