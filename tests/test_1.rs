use funlog::funlog;
use gag::BufferRedirect;
use log::info;
use std::io::Read;

#[funlog]
fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_basic_logging() {
        mock_logger::init();
        hello();
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            assert_eq!(entries[0].body, "hello() start");
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "hello() end");
        });
    }
}
