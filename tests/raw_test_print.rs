use funlog::funlog;
use gag::BufferRedirect;
use std::io::Read;

#[funlog(print)]
fn hello() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_basic_logging() {
        let mut buf = BufferRedirect::stdout().unwrap();
        hello();
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(&output[..], "hello [in ]\nHello!\nhello [out]\n");
    }
}
