use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn get_solution(outside_chars: Vec<char>, inside_char:char) -> Vec<String> {
    let (sanitized_outside_chars, sanitized_inside_char) = sanitize_input(&outside_chars, &inside_char).unwrap();
    validate_input(&sanitized_outside_chars, &sanitized_inside_char).unwrap();

    let hash_map_of_valid_words = process_word_list();

    let permutations = get_permutations(sanitized_outside_chars, sanitized_inside_char);

    let mut result: Vec<String> = vec![];

    for p in permutations {
        if let Some(words) = hash_map_of_valid_words.get(&p) {
            for w in words {
                result.push(w.to_string());
            }
        }
    }

    result
}

pub fn sanitize_and_validate(outside_chars: Vec<char>, inside_char: char) -> (Vec<char>, char) {
    let (outside_char, inside_char) = sanitize_input(&outside_chars, &inside_char).unwrap();
    validate_input(&outside_char, &inside_char).unwrap();

    (outside_chars, inside_char)
}

fn sanitize_input(outside_chars: &Vec<char>, inside_char: &char) -> Result<(Vec<char>, char), String> {
    let mut unique_outside_chars = HashSet::new();
    outside_chars.iter().for_each(|&c|{
        unique_outside_chars.insert(c.to_lowercase().next().unwrap());
    });

    if outside_chars.len() != unique_outside_chars.len() {
        return Err("all outside characters must be unique".to_string());
    }

    let mut outside_chars: Vec<char> = unique_outside_chars.into_iter().collect();
    outside_chars.sort();

    let inside_char = inside_char.to_lowercase().next().unwrap();
    Ok((outside_chars, inside_char))
}

fn validate_input(outside_chars: &Vec<char>, inside_char: &char) -> Result<(), String> {
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

    Ok(())
}

pub fn get_permutations(outside_chars: Vec<char>, inside_char: char) -> Vec<String> {
    let mut perumtations: Vec<String> = vec![];

    for length in 0..=outside_chars.len() {
        let mut current_combination = Vec::with_capacity(length);
        generate_combinations(&outside_chars, length, 0, &mut current_combination, &mut perumtations);
    };

    perumtations.iter_mut().for_each(|s| {
        s.push(inside_char);
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        *s = String::from_iter(chars);
    });

    perumtations
}

fn generate_combinations(
    chars: &[char],
    length: usize,
    start: usize,
    current_combinations: &mut Vec<char>,
    perumtations: &mut Vec<String>,
) {
    if current_combinations.len() == length {
        perumtations.push(current_combinations.iter().collect());
        return;
    }

    for i in start..chars.len() {
        current_combinations.push(chars[i]);
        generate_combinations(chars, length, i + 1, current_combinations, perumtations);
        current_combinations.pop();
    }
}

pub fn process_word_list() -> HashMap<String, Vec<String>> {
    let mut processed_words:HashMap<String, Vec<String>> = HashMap::new();

    let path_to_word_file = Path::new("/usr/share/dict/words");
    let word_list = std::fs::read_to_string(path_to_word_file).expect("Unable to read file");

    let words = word_list.split("\n");

    for w in words {
        if w.len() < 4 {
            continue
        }

        let w = w.to_lowercase();

        let mut unique_chars = HashSet::new();
        for x in w.chars() {
            unique_chars.insert(x);
        };

        let mut chars: Vec<char> = unique_chars.into_iter().collect();
        chars.sort();

        // join the chars into a string name key
        let key = chars.iter().collect::<String>();

        processed_words.entry(key).or_insert(Vec::new()).push(w.to_string());
    }

    processed_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_input_returns_input_happy_path() {
        let outside_chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let inside_char = 'g';

        let result = validate_input(&outside_chars, &inside_char);

        match result {
            Ok(_) => (),
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

        let result = sanitize_input(&outside_chars, &inside_char);

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
        assert_eq!(sanitize_input(&input, &inside_char), Err("all outside characters must be unique".to_string()));
    }

    #[test]
    fn sanitize_input_sorts_outside_chars() {
        let input_outside_chars = vec!['b','a'];
        let input_inside_char = 'c';

        let (sanitized_outside_chars, sanitized_inside_char) = sanitize_input(&input_outside_chars, &input_inside_char).unwrap();
        assert_eq!(vec!['a', 'b'], sanitized_outside_chars);
        assert_eq!('c', sanitized_inside_char);
    }
}