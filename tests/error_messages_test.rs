// 这个文件用于测试改进的错误提示
// 注意：这些测试用例会编译失败，用于验证错误信息的质量

#[cfg(test)]
mod error_message_tests {
    use funlog::funlog;

    // 测试重复的日志级别配置
    // #[funlog(debug, info)]
    // fn test_duplicate_log_level() {}

    // 测试重复的参数配置
    // #[funlog(all, none)]
    // fn test_duplicate_param_config() {}

    // 测试无效的参数名
    // #[funlog(params(invalid_param))]
    // fn test_invalid_param(valid_param: i32) {}

    // 测试拼写错误的属性
    // #[funlog(debgu)]  // debug 的拼写错误
    // fn test_typo_attribute() {}

    // #[funlog(prnit)]  // print 的拼写错误
    // fn test_typo_print() {}

    // #[funlog(onStrat)]  // onStart 的拼写错误
    // fn test_typo_position() {}

    // 测试无效的属性格式
    // #[funlog(params)]  // 缺少参数列表
    // fn test_invalid_params_format() {}

    // 测试冲突的位置配置
    // #[funlog(onStart, onEnd)]
    // fn test_conflicting_positions() {}

    #[test]
    fn test_valid_configurations() {
        // 这些是正确的配置，应该能正常编译
        
        #[funlog(debug)]
        fn valid_simple() {}

        #[funlog(info, all)]
        fn valid_with_all_params(a: i32, b: i32) {}

        #[funlog(warn, params(a))]
        fn valid_with_specific_params(a: i32, b: i32) {}

        #[funlog(error, onStart, retVal)]
        fn valid_complex() -> i32 { 42 }

        // 调用函数以避免未使用警告
        valid_simple();
        valid_with_all_params(1, 2);
        valid_with_specific_params(1, 2);
        let _ = valid_complex();
    }
}