use funlog::funlog;

fn main() {
    let result = test();
    println!("Final result: {}", result);
}

#[funlog(print, retVal)]
fn test() -> i32 {
    println!("Generating random number...");
    42
}