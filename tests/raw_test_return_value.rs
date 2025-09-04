use funlog::funlog;
use std::env::set_var;

#[funlog(debug, retVal)]
fn square(x: i32) -> i32 {
    x * x
}

#[funlog(debug, retVal, onEnd)]
fn cube(x: i32) -> i32 {
    x * x * x
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_return_value_logging() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        let result = square(5);
        assert_eq!(result, 25);
        
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "square [in ]");
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "square [out]: return:25");
        });
    }

    #[test]
    fn test_return_value_on_end_logging() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        let result = cube(3);
        assert_eq!(result, 27);
        
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "cube [out]: return:27");
        });
    }
}