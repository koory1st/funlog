use funlog::funlog;
use gag::BufferRedirect;

#[funlog(debug)]
fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_logging() {
        gag::BufferRedirect::stdout().unwrap();
        hello();
    }
}
