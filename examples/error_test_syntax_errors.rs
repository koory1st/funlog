// 测试语法错误的错误提示
// 运行: cargo check --example error_test_syntax_errors
// 预期错误: 语法错误和格式建议

use funlog::funlog;

fn main() {
    println!("这个示例用于测试语法错误的错误提示");
    println!("预期会看到语法错误和格式建议");
}

// 错误示例 1: params 写成 param
#[funlog(param(a))]
fn test_param_instead_of_params(a: i32) {
    println!("param 应该是 params: {}", a);
}

// 错误示例 2: 使用了不支持的列表格式
#[funlog(level(debug))]
fn test_unsupported_list_format() {
    println!("不支持的列表格式");
}

// 错误示例 3: 空的 params 列表
#[funlog(params())]
fn test_empty_params_list(a: i32, b: i32) {
    println!("空的参数列表: {} {}", a, b);
}

// 错误示例 4: params 中使用了非标识符
// 注意：这个可能会在更早的解析阶段失败
// #[funlog(params("string_literal"))]
// fn test_non_identifier_in_params(a: i32) {
//     println!("参数中使用了字符串字面量: {}", a);
// }

// 错误示例 5: 使用了复杂的嵌套结构
#[funlog(config(debug, params(a)))]
fn test_nested_structure(a: i32) {
    println!("嵌套结构: {}", a);
}

// 错误示例 6: 混合使用正确和错误的语法
#[funlog(debug, param(a), info)]
fn test_mixed_syntax(a: i32) {
    println!("混合语法: {}", a);
}

// 错误示例 7: 使用了看起来像参数但格式错误的配置
#[funlog(parameters(a, b))]
fn test_parameters_typo(a: i32, b: i32) {
    println!("parameters 拼写错误: {} {}", a, b);
}

// 错误示例 8: 使用了看起来像配置但不存在的选项
#[funlog(logging(debug))]
fn test_logging_config(a: i32) {
    println!("logging 配置不存在: {}", a);
}
