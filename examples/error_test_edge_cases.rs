// 测试边缘情况的错误提示
// 运行: cargo check --example error_test_edge_cases
// 预期错误: 各种边缘情况的错误信息

use funlog::funlog;

fn main() {
    println!("这个示例用于测试边缘情况的错误提示");
    println!("预期会看到各种边缘情况的错误信息");
}

// 错误示例 1: 完全未知的属性
#[funlog(completely_unknown_attribute)]
fn test_unknown_attribute() {
    println!("完全未知的属性");
}

// 错误示例 2: 数字作为属性（如果可能的话）
// #[funlog(123)]
// fn test_numeric_attribute() {
//     println!("数字属性");
// }

// 错误示例 3: 很长的拼写错误
#[funlog(debuggingmode)]
fn test_long_typo() {
    println!("很长的拼写错误");
}

// 错误示例 4: 大小写错误
#[funlog(DEBUG)]
fn test_case_error() {
    println!("大小写错误");
}

// 错误示例 5: 使用了其他编程语言的关键字
#[funlog(console)]
fn test_other_language_keyword() {
    println!("其他语言的关键字");
}

// 错误示例 6: 使用了看起来合理但不存在的组合
#[funlog(debugLevel)]
fn test_camel_case_combo() {
    println!("驼峰命名的组合");
}

// 错误示例 7: 使用了下划线分隔的属性
#[funlog(debug_mode)]
fn test_underscore_attribute() {
    println!("下划线分隔的属性");
}

// 错误示例 8: 使用了连字符分隔的属性
// 注意：这可能在词法分析阶段就失败
// #[funlog(debug-mode)]
// fn test_hyphen_attribute() {
//     println!("连字符分隔的属性");
// }

// 错误示例 9: 非常相似但错误的属性
#[funlog(retValue)] // retVal 的变体
fn test_similar_but_wrong() -> i32 {
    println!("非常相似但错误的属性");
    42
}

// 错误示例 10: 使用了复数形式
#[funlog(errors)] // error 的复数
fn test_plural_form() {
    println!("复数形式的属性");
}

// 错误示例 11: 缺少字母的属性
#[funlog(deb)] // debug 缺少字母
fn test_missing_letters() {
    println!("缺少字母的属性");
}

// 错误示例 12: 多余字母的属性
#[funlog(debugg)] // debug 多了字母
fn test_extra_letters() {
    println!("多余字母的属性");
}
