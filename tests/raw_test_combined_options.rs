use funlog::funlog;
use std::env::set_var;

#[funlog(info, all, retVal, onStartEnd)]
fn complex_function(name: &str, age: u32, active: bool) -> String {
    format!(
        "{} is {} years old and {}",
        name,
        age,
        if active { "active" } else { "inactive" }
    )
}

#[funlog(warn, params(x), retVal, onEnd)]
fn power(x: i32, y: i32) -> i32 {
    x.pow(y as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_complex_combined_options() {
        unsafe {
            set_var("RUST_LOG", "info");
        }
        mock_logger::init();
        let result = complex_function("Alice", 30, true);
        assert_eq!(result, "Alice is 30 years old and active");

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(
                entries[0].body,
                "complex_function [in ]: name:\"Alice\", age:30, active:true"
            );
            assert_eq!(entries[1].level, log::Level::Info);
            assert_eq!(
                entries[1].body,
                "complex_function [out]: return:\"Alice is 30 years old and active\""
            );
        });
    }

    #[test]
    fn test_params_with_return_on_end() {
        unsafe {
            set_var("RUST_LOG", "warn");
        }
        mock_logger::init();
        let result = power(2, 8);
        assert_eq!(result, 256);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].level, log::Level::Warn);
            assert_eq!(entries[0].body, "power [out]: x:2, return:256");
        });
    }
}
