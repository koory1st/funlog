use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "info");
    }
    env_logger::init();
    test();
}

#[funlog(info)]
fn test() -> i32 {
    println!("Hello, world!");
    0
}
