use funlog::funlog;

fn main() {
    let result = test(1, 2);
    println!("Result: {result}");
}

#[funlog(print, none)]
fn test(a: i32, b: i32) -> i32 {
    println!("Hello, world! a={a}, b={b}");
    a + b
}
