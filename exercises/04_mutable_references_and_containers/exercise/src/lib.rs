use require_lifetimes::require_lifetimes;

/// This function takes in a "vector" of `&strs`, a "loc" `usize`
/// and a "new" `&str`. Your job is to replace the old string at the
/// location (i.e., array index) "loc" with the "new" one.  Don't do
/// anything if "loc" is beyond the end of "vector".
///
/// Make sure it passes this test:
///
/// ```rust
/// use ex04::vector_set;
///
///
/// // Create a vector of strings.
/// let strings = vec!["Hello".to_string(), "My".to_string(), "Name".to_string(), "Is".to_string(), "Tom".to_string()];
///
/// // Create some strings to replace inside that vector.
/// let your = "Your".to_string();
/// let unknown = "Unknown".to_string();
///
///
/// // Create a vector of references to the string vector.
/// let mut message: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
///
/// // Set some references
/// vector_set(&mut message, 1, &your);
/// vector_set(&mut message, 4, &unknown);
/// vector_set(&mut message, 10, &unknown);
///
/// // Hopefully, they're now equal
/// assert_eq!(message , vec!["Hello", "Your", "Name", "Is", "Unknown"]);
/// ````
#[require_lifetimes(!)]
pub fn vector_set<'content, 'vec>(
    vector: &'vec mut Vec<&'content str>,
    loc: usize,
    new: &'content str,
) {
    if vector.len() > loc {
        vector[loc] = new
    }
}

#[cfg(test)]
mod tests {
    use super::vector_set;

    #[test]
    fn test_vector_set() {
        // Create a vector of strings.
        let strings = vec![
            "Hello".to_string(),
            "My".to_string(),
            "Name".to_string(),
            "Is".to_string(),
            "Tom".to_string(),
        ];

        // Create some strings to replace inside that vector.
        let your = "Your".to_string();
        let unknown = "Unknown".to_string();

        // Create a vector of references to the string vector.
        let mut message: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

        // Set some references
        vector_set(&mut message, 1, &your);
        vector_set(&mut message, 4, &unknown);
        vector_set(&mut message, 10, &unknown);

        // Hopefully, they're now equal
        assert_eq!(message, vec!["Hello", "Your", "Name", "Is", "Unknown"]);
    }
    #[test]
    fn test_vector_set_empty_vector() {
        let mut empty_vector: Vec<&str> = vec![];
        let test_str = "test".to_string();
        
        vector_set(&mut empty_vector, 0, &test_str);
        
        assert_eq!(empty_vector, Vec::<&str>::new());
    }

    #[test]
    fn test_vector_set_single_element() {
        let original = "original".to_string();
        let replacement = "replacement".to_string();
        let mut vector = vec![original.as_str()];
        
        vector_set(&mut vector, 0, &replacement);
        
        assert_eq!(vector, vec!["replacement"]);
    }

    #[test]
    fn test_vector_set_out_of_bounds() {
        let strings = vec!["one".to_string(), "two".to_string()];
        let replacement = "new".to_string();
        let mut vector: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        let original_vector = vector.clone();
        
        vector_set(&mut vector, 5, &replacement);
        
        assert_eq!(vector, original_vector);
    }

    #[test]
    fn test_vector_set_boundary_conditions() {
        let strings = vec!["first".to_string(), "middle".to_string(), "last".to_string()];
        let new_first = "NEW_FIRST".to_string();
        let new_last = "NEW_LAST".to_string();
        let mut vector: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        
        // Test first element
        vector_set(&mut vector, 0, &new_first);
        // Test last element
        vector_set(&mut vector, 2, &new_last);
        
        assert_eq!(vector, vec!["NEW_FIRST", "middle", "NEW_LAST"]);
    }

    #[test]
    fn test_vector_set_same_value_multiple_times() {
        let strings = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let replacement = "X".to_string();
        let mut vector: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();
        
        vector_set(&mut vector, 0, &replacement);
        vector_set(&mut vector, 1, &replacement);
        vector_set(&mut vector, 2, &replacement);
        
        assert_eq!(vector, vec!["X", "X", "X"]);
    }
}
