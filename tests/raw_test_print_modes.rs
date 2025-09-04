use funlog::funlog;
use gag::BufferRedirect;
use std::io::Read;

#[funlog(print, all)]
fn print_with_params(name: &str, count: i32) -> String {
    format!("Hello {name} ({count})")
}

#[funlog(print, retVal)]
fn print_with_return(x: i32) -> i32 {
    x * x
}

#[funlog(print, all, retVal)]
fn print_with_both(a: i32, b: i32) -> i32 {
    a + b
}

#[funlog(print, onStart)]
fn print_start_only(msg: &str) {
    println!("Processing: {msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Print tests are ignored by default as they require stdout capture
    fn test_print_with_params() {
        let mut buf = BufferRedirect::stdout().unwrap();
        let result = print_with_params("Alice", 5);
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(result, "Hello Alice (5)");
        assert!(output.contains("print_with_params [in ]: name:Alice, count:5"));
        assert!(output.contains("print_with_params [out]"));
    }

    #[test]
    #[ignore]
    fn test_print_with_return() {
        let mut buf = BufferRedirect::stdout().unwrap();
        let result = print_with_return(7);
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(result, 49);
        assert!(output.contains("print_with_return [in ]"));
        assert!(output.contains("print_with_return [out]: return:49"));
    }

    #[test]
    #[ignore]
    fn test_print_with_both() {
        let mut buf = BufferRedirect::stdout().unwrap();
        let result = print_with_both(10, 15);
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert_eq!(result, 25);
        assert!(output.contains("print_with_both [in ]: a:10, b:15"));
        assert!(output.contains("print_with_both [out]: return:25"));
    }

    #[test]
    #[ignore]
    fn test_print_start_only() {
        let mut buf = BufferRedirect::stdout().unwrap();
        print_start_only("test message");
        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();

        assert!(output.contains("print_start_only [in ]: msg:test message"));
        assert!(output.contains("Processing: test message"));
        // Should not contain [out] message
        assert!(!output.contains("print_start_only [out]"));
    }
}
