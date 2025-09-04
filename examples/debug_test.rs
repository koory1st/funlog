use funlog::funlog;

fn main() {
    test_no_params();
    test_with_params(1, 2);
}

#[funlog(print)]
fn test_no_params() -> i32 {
    println!("No params function");
    42
}

#[funlog(print, none)]
fn test_with_params(a: i32, b: i32) -> i32 {
    println!("With params function: {a} + {b}");
    a + b
}
