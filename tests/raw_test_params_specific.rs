use funlog::funlog;
use std::env::set_var;

#[funlog(debug, params(a, c))]
fn calculate(a: i32, b: i32, c: i32) -> i32 {
    a + b * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_params_specific_logging() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        let result = calculate(10, 5, 2);
        assert_eq!(result, 20);
        
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "calculate [in ]: a:10, c:2");
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "calculate [out]");
        });
    }
}