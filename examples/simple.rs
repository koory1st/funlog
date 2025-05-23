use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "trace");
    }
    env_logger::init();
    test();
}

#[funlog(trace)]
fn test() {
    println!("Hello, world!");
}
