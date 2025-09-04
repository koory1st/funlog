use funlog::funlog;

fn main() {
    test(1, 2);
}

#[funlog(print)]
fn test(a: i32, b: i32) -> i32 {
    println!("Simple test: {a} + {b}");
    a + b
}
