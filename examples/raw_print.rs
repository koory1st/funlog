use funlog::funlog;

fn main() {
    test();
}

#[funlog(print)]
fn test() -> i32 {
    println!("Hello, world!");
    0
}