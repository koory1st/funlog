use funlog::funlog;

fn main() {
    test(100, 200);
}

#[funlog(print, all)]
fn test(x: i32, y: i32) -> i32 {
    println!("Calculating: {} * {}", x, y);
    x * y
}