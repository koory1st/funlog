// 这个文件展示了 funlog 宏的错误提示改进
// 取消注释下面的代码来查看具体的错误信息

use funlog::funlog;

fn main() {
    println!("这个文件用于展示 funlog 的错误提示");
    println!("取消注释下面的代码来查看具体的错误信息");
}

// 1. 重复的日志级别配置
// #[funlog(debug, info)]
// fn duplicate_log_level() {
//     println!("这会产生冲突错误");
// }

// 2. 重复的参数配置
// #[funlog(all, none)]
// fn duplicate_param_config(a: i32) {
//     println!("参数配置冲突");
// }

// 3. 无效的参数名
// #[funlog(params(nonexistent))]
// fn invalid_param_name(existing: i32) {
//     println!("参数名不存在");
// }

// 4. 拼写错误的属性 - debug 写成 debgu
// #[funlog(debgu)]
// fn typo_debug() {
//     println!("拼写错误会得到建议");
// }

// 5. 拼写错误的属性 - print 写成 prnit
// #[funlog(prnit)]
// fn typo_print() {
//     println!("拼写错误会得到建议");
// }

// 6. 拼写错误的位置控制 - onStart 写成 onStrat
// #[funlog(onStrat)]
// fn typo_position() {
//     println!("位置控制拼写错误");
// }

// 7. 参数语法错误 - params 写成 param
// #[funlog(param(a))]
// fn wrong_params_syntax(a: i32) {
//     println!("参数语法错误");
// }

// 8. 冲突的位置配置
// #[funlog(onStart, onEnd)]
// fn conflicting_positions() {
//     println!("位置配置冲突");
// }

// 9. 空的参数列表但函数有参数
// #[funlog(params())]
// fn empty_params_with_args(a: i32, b: i32) {
//     println!("空参数列表但函数有参数");
// }

// 10. 正确的用法示例
#[funlog(debug, params(name))]
fn correct_usage(name: &str, age: u32) {
    println!("Hello, {} (age: {})", name, age);
}

// 调用正确的函数
#[allow(dead_code)]
fn demo_correct_usage() {
    correct_usage("Alice", 30);
}
