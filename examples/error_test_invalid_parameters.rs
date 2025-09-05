// 测试无效参数名的错误提示
// 运行: cargo check --example error_test_invalid_parameters
// 预期错误: funlog 参数错误: 参数 'xxx' 不存在

use funlog::funlog;

fn main() {
    println!("这个示例用于测试无效参数名的错误提示");
    println!("预期会看到参数不存在的错误信息和可用参数列表");
}

// 错误示例 1: 参数名不存在
#[funlog(params(nonexistent_param))]
fn test_invalid_param_name(valid_param: i32, another_param: &str) {
    println!("valid_param: {}, another_param: {}", valid_param, another_param);
}

// 错误示例 2: 部分参数名错误
#[funlog(params(valid_param, wrong_param))]
fn test_partial_invalid_params(valid_param: i32, another_param: &str) {
    println!("valid_param: {}, another_param: {}", valid_param, another_param);
}

// 错误示例 3: 在没有参数的函数中使用 params
#[funlog(params(some_param))]
fn test_params_on_no_param_function() {
    println!("这个函数没有参数");
}

// 错误示例 4: 多个错误的参数名
#[funlog(params(wrong1, wrong2, wrong3))]
fn test_multiple_invalid_params(real_param: bool) {
    println!("real_param: {}", real_param);
}