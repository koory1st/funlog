use funlog::funlog;
use std::env::set_var;

#[funlog(warn)]
fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_warn_logging() {
        unsafe {
            set_var("RUST_LOG", "warn");
        }
        mock_logger::init();
        hello();
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Warn);
            assert_eq!(entries[0].body, "hello [in ]");
            assert_eq!(entries[1].level, log::Level::Warn);
            assert_eq!(entries[1].body, "hello [out]");
        });
    }
}
