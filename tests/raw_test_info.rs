use funlog::funlog;
use std::env::set_var;

#[funlog(info)]
fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_info_logging() {
        unsafe {
            set_var("RUST_LOG", "info");
        }
        mock_logger::init();
        hello();
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(entries[0].body, "hello [in ]");
            assert_eq!(entries[1].level, log::Level::Info);
            assert_eq!(entries[1].body, "hello [out]");
        });
    }
}
