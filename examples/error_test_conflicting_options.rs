// 测试配置冲突的错误提示
// 运行: cargo check --example error_test_conflicting_options
// 预期错误: 配置冲突错误信息

use funlog::funlog;

fn main() {
    println!("这个示例用于测试配置冲突的错误提示");
    println!("预期会看到配置冲突的错误信息");
}

// 错误示例 1: 参数配置冲突 - all 和 none
#[funlog(all, none)]
fn test_param_config_conflict1(param: i32) {
    println!("参数配置冲突: {}", param);
}

// 错误示例 2: 参数配置冲突 - all 和 params
#[funlog(all, params(param))]
fn test_param_config_conflict2(param: i32) {
    println!("参数配置冲突: {}", param);
}

// 错误示例 3: 参数配置冲突 - none 和 params
#[funlog(none, params(param))]
fn test_param_config_conflict3(param: i32) {
    println!("参数配置冲突: {}", param);
}

// 错误示例 4: 位置配置冲突 - onStart 和 onEnd
#[funlog(onStart, onEnd)]
fn test_position_conflict1() {
    println!("位置配置冲突");
}

// 错误示例 5: 位置配置冲突 - onStart 和 onStartEnd
#[funlog(onStart, onStartEnd)]
fn test_position_conflict2() {
    println!("位置配置冲突");
}

// 错误示例 6: 位置配置冲突 - onEnd 和 onStartEnd
#[funlog(onEnd, onStartEnd)]
fn test_position_conflict3() {
    println!("位置配置冲突");
}

// 错误示例 7: 复杂的冲突 - 多个日志级别 + 参数冲突
#[funlog(debug, info, all, none)]
fn test_multiple_conflicts(param: i32) {
    println!("多重配置冲突: {}", param);
}

// 错误示例 8: 重复的 retVal 配置（虽然语法上不可能，但测试逻辑）
// 注意：这个实际上不会产生我们期望的错误，因为 retVal 是布尔值
#[funlog(debug, all, none)]
fn test_retval_with_conflicts(param: i32) -> i32 {
    println!("retVal 相关冲突: {}", param);
    param * 2
}