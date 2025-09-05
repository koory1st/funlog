// 测试重复日志级别配置的错误提示
// 运行: cargo check --example error_test_duplicate_log_levels
// 预期错误: funlog 配置冲突: 'debug' 和 'info' 不能同时使用

use funlog::funlog;

fn main() {
    println!("这个示例用于测试重复日志级别配置的错误提示");
    println!("预期会看到配置冲突的错误信息");
}

// 错误示例 1: 同时使用两个日志级别
#[funlog(debug, info)]
fn test_duplicate_log_levels() {
    println!("这个函数不会被编译");
}

// 错误示例 2: 同时使用三个日志级别
#[funlog(trace, debug, warn)]
fn test_multiple_log_levels() {
    println!("这个函数也不会被编译");
}

// 错误示例 3: print 和其他日志级别冲突
#[funlog(print, error)]
fn test_print_conflict() {
    println!("print 和 error 不能同时使用");
}