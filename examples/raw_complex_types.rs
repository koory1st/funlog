use funlog::funlog;
use std::env::set_var;

fn main() {
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // 使用简单类型的示例
    let result = calculate_area(10, 20);
    println!("Area: {result}");

    let name = "Alice".to_string();
    let age = 30;
    process_user_data(&name, age);
}

// 记录所有简单类型参数和返回值
#[funlog(debug, all, retVal)]
fn calculate_area(width: i32, height: i32) -> i32 {
    println!("Calculating area for {width}x{height}");
    width * height
}

// 记录部分参数（字符串引用可以正常显示）
#[funlog(debug, params(name, age))]
fn process_user_data(name: &str, age: u32) {
    println!("Processing user: {name} (age: {age})");
}
