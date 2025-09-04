use funlog::funlog;
use std::collections::HashMap;
use std::env::set_var;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User(id:{}, name:{})", self.id, self.name)
    }
}

#[funlog(debug, params(user))]
fn process_user(user: &User, metadata: HashMap<String, String>) -> bool {
    println!("Processing user: {user:?} with metadata: {metadata:?}");
    true
}

#[funlog(info, params(multiplier))]
fn sum_vector(numbers: &[i32], multiplier: f64) -> f64 {
    numbers.iter().sum::<i32>() as f64 * multiplier
}

#[funlog(warn, retVal)]
fn create_user(id: u32, name: String) -> User {
    User { id, name }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock_logger::MockLogger;

    #[test]
    fn test_complex_types_logging() {
        unsafe {
            set_var("RUST_LOG", "debug");
        }
        mock_logger::init();

        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };
        let mut metadata = HashMap::new();
        metadata.insert("role".to_string(), "admin".to_string());

        let result = process_user(&user, metadata);
        assert!(result);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Debug);
            // Note: Only user parameter is logged due to params(user)
            assert_eq!(
                entries[0].body,
                "process_user [in ]: user:User(id:1, name:Alice)"
            );
            assert_eq!(entries[1].level, log::Level::Debug);
            assert_eq!(entries[1].body, "process_user [out]");
        });
    }

    #[test]
    fn test_vector_params_logging() {
        unsafe {
            set_var("RUST_LOG", "info");
        }
        mock_logger::init();

        let numbers = vec![1, 2, 3, 4, 5];
        let result = sum_vector(&numbers, 2.5);
        assert_eq!(result, 37.5);

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(entries[0].body, "sum_vector [in ]: multiplier:2.5");
            assert_eq!(entries[1].level, log::Level::Info);
            assert_eq!(entries[1].body, "sum_vector [out]");
        });
    }

    #[test]
    fn test_struct_return_value() {
        unsafe {
            set_var("RUST_LOG", "warn");
        }
        mock_logger::init();

        let user = create_user(42, "Bob".to_string());
        assert_eq!(user.id, 42);
        assert_eq!(user.name, "Bob");

        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 2);
            assert_eq!(entries[0].level, log::Level::Warn);
            assert_eq!(entries[0].body, "create_user [in ]");
            assert_eq!(entries[1].level, log::Level::Warn);
            assert_eq!(
                entries[1].body,
                "create_user [out]: return:User(id:42, name:Bob)"
            );
        });
    }
}
