// 这个测试文件验证错误处理的改进
// 注意：某些测试被注释掉了，因为它们会导致编译错误

#[cfg(test)]
mod tests {
    use funlog::funlog;

    #[test]
    fn test_valid_usage() {
        // 这些是正确的用法，应该能正常工作
        
        #[funlog(debug)]
        fn simple_debug() {}
        
        #[funlog(info, all)]
        fn with_all_params(a: i32, b: i32) {}
        
        #[funlog(warn, params(name))]
        fn with_specific_param(name: &str, age: u32) {}
        
        #[funlog(error, onStart, retVal)]
        fn with_return_value() -> i32 { 42 }
        
        // 调用函数
        simple_debug();
        with_all_params(1, 2);
        with_specific_param("test", 25);
        let _ = with_return_value();
    }

    // 下面的测试用例会产生编译错误，用于验证错误信息的质量
    // 取消注释来查看具体的错误信息

    /*
    // 测试 1: 重复的日志级别配置
    #[test]
    fn test_duplicate_log_levels() {
        #[funlog(debug, info)]  // 应该显示冲突错误
        fn duplicate_levels() {}
        
        duplicate_levels();
    }

    // 测试 2: 无效的参数名
    #[test] 
    fn test_invalid_parameter() {
        #[funlog(params(nonexistent))]  // 应该显示参数不存在错误
        fn invalid_param(existing: i32) {}
        
        invalid_param(42);
    }

    // 测试 3: 拼写错误
    #[test]
    fn test_spelling_errors() {
        #[funlog(debgu)]  // debug 的拼写错误，应该提供建议
        fn typo_debug() {}
        
        #[funlog(prnit)]  // print 的拼写错误，应该提供建议  
        fn typo_print() {}
        
        #[funlog(onStrat)]  // onStart 的拼写错误，应该提供建议
        fn typo_position() {}
        
        typo_debug();
        typo_print();
        typo_position();
    }

    // 测试 4: 参数配置冲突
    #[test]
    fn test_parameter_conflicts() {
        #[funlog(all, none)]  // 应该显示参数配置冲突
        fn param_conflict(a: i32) {}
        
        param_conflict(1);
    }

    // 测试 5: 位置配置冲突
    #[test]
    fn test_position_conflicts() {
        #[funlog(onStart, onEnd)]  // 应该显示位置配置冲突
        fn position_conflict() {}
        
        position_conflict();
    }

    // 测试 6: 语法错误
    #[test]
    fn test_syntax_errors() {
        #[funlog(param(a))]  // params 写成 param，应该提供建议
        fn syntax_error(a: i32) {}
        
        syntax_error(1);
    }
    */
}

// 用于手动测试的函数
#[allow(dead_code)]
fn manual_test_examples() {
    println!("=== Funlog 错误处理改进演示 ===");
    println!("要查看错误信息，请取消注释测试用例并运行 cargo test");
    println!();
    println!("改进的错误信息特性：");
    println!("1. 中文错误提示");
    println!("2. 具体的修复建议");
    println!("3. 拼写错误自动建议");
    println!("4. 冲突检测和说明");
    println!("5. 上下文相关的帮助信息");
}