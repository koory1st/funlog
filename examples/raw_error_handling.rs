use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    match divide(10, 2) {
        Ok(result) => println!("Success: {result}"),
        Err(e) => println!("Error: {e}"),
    }

    match divide(10, 0) {
        Ok(result) => println!("Success: {result}"),
        Err(e) => println!("Error: {e}"),
    }
}

#[funlog(debug, all)]
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
