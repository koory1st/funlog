use funlog::funlog;
use std::env::set_var;

#[funlog(debug, all)]
fn print_message(msg: &str, count: u32) {
    for i in 0..count {
        println!("{i}: {msg}");
    }
}

#[funlog(info, none, onStart)]
fn initialize_system() {
    // System initialization logic
    println!("System initialized");
}

#[funlog(warn, params(level), onEnd)]
fn log_event(level: &str, message: &str) {
    println!("[{level}] {message}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_void_function_with_params() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();
        print_message("test", 2);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(
                entries[0].body,
                "print_message [in ]: msg:\"test\", count:2"
            );
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "print_message [out]");
        });
    }

    #[test]
    fn test_void_function_on_start_only() {
        unsafe {
            set_var("RUST_LOG", "info");
        }
        mock_logger::init();
        initialize_system();

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(entries[0].body, "initialize_system [in ]");
        });
    }

    #[test]
    fn test_void_function_params_on_end() {
        unsafe {
            set_var("RUST_LOG", "warn");
        }
        mock_logger::init();
        log_event("ERROR", "Something went wrong");

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].level, log::Level::Warn);
            assert_eq!(entries[0].body, "log_event [out]: level:\"ERROR\"");
        });
    }
}
