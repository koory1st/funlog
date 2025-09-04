use funlog::funlog;
use std::env::set_var;

// Test function with no parameters
#[funlog(debug)]
fn no_params() -> i32 {
    42
}

// Test function with single parameter
#[funlog(info, all)]
fn single_param(x: i32) -> i32 {
    x * 2
}

// Test function with many parameters
#[allow(clippy::too_many_arguments)]
#[funlog(warn, params(a, e, h))]
fn many_params(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    a + b + c + d + e + f + g + h
}

// Test function with string parameter (simpler than generics)
#[funlog(error, all)]
fn string_function(value: String) -> String {
    value.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_no_params_function() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        let result = no_params();
        assert_eq!(result, 42);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "no_params [in ]");
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "no_params [out]");
        });
    }

    #[test]
    fn test_single_param_function() {
        unsafe {
            set_var("RUST_LOG", "info");
        }
        mock_logger::init();
        let result = single_param(21);
        assert_eq!(result, 42);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(entries[0].body, "single_param [in ]: x:21");
            assert_eq!(entries[1].level, log::Level::Info);
            assert_eq!(entries[1].body, "single_param [out]");
        });
    }

    #[test]
    fn test_many_params_selective() {
        unsafe {
            set_var("RUST_LOG", "warn");
        }
        mock_logger::init();
        let result = many_params(1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(result, 36);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Warn);
            assert_eq!(entries[0].body, "many_params [in ]: a:1, e:5, h:8");
            assert_eq!(entries[1].level, log::Level::Warn);
            assert_eq!(entries[1].body, "many_params [out]");
        });
    }

    #[test]
    fn test_string_function() {
        unsafe {
            set_var("RUST_LOG", "error");
        }
        mock_logger::init();
        let result = string_function("test".to_string());
        assert_eq!(result, "TEST");

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Error);
            assert_eq!(entries[0].body, "string_function [in ]: value:test");
            assert_eq!(entries[1].level, log::Level::Error);
            assert_eq!(entries[1].body, "string_function [out]");
        });
    }
}
