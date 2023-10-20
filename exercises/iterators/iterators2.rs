// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.
 

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),  
        Some(first) => {
            let capitalized = first.to_uppercase().collect::<String>();
            capitalized + c.as_str()  
        }
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut capitalized_words = Vec::new();
    for word in words {
        capitalized_words.push(capitalize_first(word));
    }
    capitalized_words
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut capitalized_words = String::new();
    for word in words {
        let capitalized_word = capitalize_first(word);
        if !capitalized_word.is_empty() {
            capitalized_words.push_str(&capitalized_word);
        }
    }
    capitalized_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), vec!["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
