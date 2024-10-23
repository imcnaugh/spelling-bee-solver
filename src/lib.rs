use std::collections::HashSet;

fn _hello() -> String {
    String::from("Hello, world!")
}


// TODO combine sanitize input and validate input
// Then break this into its own module
fn sanitize_input(outside_chars: Vec<char>, inside_char: char) -> Result<(Vec<char>, char), String> {
    let mut unique_outside_chars = HashSet::new();
    outside_chars.iter().for_each(|&c|{
        unique_outside_chars.insert(c.to_lowercase().next().unwrap());
    });
    if outside_chars.len() != unique_outside_chars.len() {
        return Err("all outside characters must be unique".to_string());
    }

    let outside_chars: Vec<char> = unique_outside_chars.into_iter().collect();

    let inside_char = inside_char.to_lowercase().next().unwrap();
    Ok((outside_chars, inside_char))
}

fn validate_input<'a>(outside_chars: &'a Vec<char>, inside_char: &'a char) -> Result<(&'a Vec<char>, &'a char), String> {
    if outside_chars.len() != 6 {
        return Err(format!("expected 6 outside characters, received {}", outside_chars.len()))
    }

    for c in outside_chars {
        if !c.is_alphabetic() {
            return Err("all outside characters are expected to be english letters from a to z".to_string())
        }
    }

    if outside_chars.contains(inside_char) {
        return Err("the inside character cannot be included in the outside characters".to_string())
    }

    Ok((outside_chars, inside_char))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(_hello(), "Hello, world!".to_string());
    }

    #[test]
    fn validate_input_returns_input_happy_path() {
        let outside_chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let inside_char = 'g';

        let result = validate_input(&outside_chars, &inside_char);

        match result {
            Ok((o, i)) => {
                assert_eq!(o, &outside_chars);
                assert_eq!(i, &inside_char);
            },
            Err(_) => panic!("Expected error, received Ok"),
        }
    }

    #[test]
    fn validate_input_fails_7_outside_chars() {
        let outside_chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let inside_char = 'h';

        let result = validate_input(&outside_chars, &inside_char);

        match result {
            Ok(_) => panic!("Expected error, received Ok"),
            Err(e) => assert_eq!("expected 6 outside characters, received 7", e),
        }
    }

    #[test]
    fn validate_check_inside_char_in_outside_chars() {
        let outside_chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let inside_char = 'f';

        let result = validate_input(&outside_chars, &inside_char);

        match result {
            Ok(_) => panic!("Expected error, received Ok"),
            Err(e) => assert_eq!("the inside character cannot be included in the outside characters", e),
        }
    }

    #[test]
    fn validate_input_ensure_outside_chars_are_alphabetical() {
        let outside_chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', '1'];
        let inside_char = 'h';

        let result = validate_input(&outside_chars, &inside_char);

        match result {
            Ok(_) => panic!("Expected error, received Ok"),
            Err(e) => assert_eq!("all outside characters are expected to be english letters from a to z", e),
        }
    }

    #[test]
    fn sanitize_input_set_input_to_lowercase() {
        let outside_chars = vec!['A'];
        let inside_char = 'D';

        let result = sanitize_input(outside_chars, inside_char);

        match result {
            Ok((o, i)) => {
                assert_eq!(vec!['a'], o);
                assert_eq!('d', i);
            },
            Err(e) => panic!("Expected Ok, got Error {}", e)
        }
    }

    #[test]
    fn sanitize_input_errors_on_non_unique_outside_chars() {
        let input = vec!['a', 'b', 'c', 'a'];
        let inside_char = 'd';
        assert_eq!(sanitize_input(input, inside_char), Err("all outside characters must be unique".to_string()));
    }
}