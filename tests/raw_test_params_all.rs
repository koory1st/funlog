use funlog::funlog;
use std::env::set_var;

#[funlog(debug, all)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_params_all_logging() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        let result = add(5, 3);
        assert_eq!(result, 8);
        
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "add [in ]: a:5, b:3");
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "add [out]");
        });
    }
}