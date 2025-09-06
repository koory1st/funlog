// 测试拼写错误的错误提示和建议
// 运行: cargo check --example error_test_spelling_mistakes
// 预期错误: 会提供拼写建议

use funlog::funlog;

fn main() {
    println!("这个示例用于测试拼写错误的错误提示");
    println!("预期会看到拼写建议和可用选项列表");
}

// 错误示例 1: debug 拼写错误
#[funlog(debgu)]
fn test_debug_typo() {
    println!("debug 拼写错误");
}

// 错误示例 2: print 拼写错误
#[funlog(prnit)]
fn test_print_typo() {
    println!("print 拼写错误");
}

// 错误示例 3: trace 拼写错误
#[funlog(tarce)]
fn test_trace_typo() {
    println!("trace 拼写错误");
}

// 错误示例 4: info 拼写错误
#[funlog(ifno)]
fn test_info_typo() {
    println!("info 拼写错误");
}

// 错误示例 5: warn 拼写错误
#[funlog(wran)]
fn test_warn_typo() {
    println!("warn 拼写错误");
}

// 错误示例 6: error 拼写错误
#[funlog(eror)]
fn test_error_typo() {
    println!("error 拼写错误");
}

// 错误示例 7: onStart 拼写错误
#[funlog(onStrat)]
fn test_onstart_typo() {
    println!("onStart 拼写错误");
}

// 错误示例 8: onEnd 拼写错误
#[funlog(onEdn)]
fn test_onend_typo() {
    println!("onEnd 拼写错误");
}

// 错误示例 9: retVal 拼写错误
#[funlog(retVla)]
fn test_retval_typo() -> i32 {
    println!("retVal 拼写错误");
    42
}

// 错误示例 10: all 拼写错误
#[funlog(al)]
fn test_all_typo(param: i32) {
    println!("all 拼写错误: {}", param);
}

// 错误示例 11: none 拼写错误
#[funlog(non)]
fn test_none_typo(param: i32) {
    println!("none 拼写错误: {}", param);
}
