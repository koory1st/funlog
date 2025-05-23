use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "error");
    }
    env_logger::init();
    test();
}

#[funlog(error)]
fn test() -> i32 {
    println!("Hello, world!");
    0
}
