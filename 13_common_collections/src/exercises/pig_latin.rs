pub fn pig_latin(latin: Option<String>) -> Option<String> {
    let Some(latin) = latin else {
        return None;
    };

    if latin.is_empty() {
        return None;
    }

    for character in latin.chars() {
        if !character.is_ascii_alphabetic() {
            return None;
        }
    }

    let latin = latin.to_lowercase();

    if latin.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        return Some(format!("{latin}-hay"));
    } else {
        let mut chars = latin.chars();
        let first_char = chars.next().unwrap();
        let rest_of_word = chars.as_str();
        return Some(format!("{rest_of_word}-{first_char}ay"));
    }
}

#[cfg(test)]
mod unit_tests_exercises_2 {
    use super::*;

    #[test]
    fn empty_string_returns_none() {
        let empty_string = Some(String::new());
        assert_eq!(pig_latin(empty_string), None);
    }

    #[test]
    fn consonant_word_moves_first_char_to_end() {
        let consonant_word = Some(String::from("cat"));
        assert_eq!(pig_latin(consonant_word), Some(String::from("at-cay")));
    }

    #[test]
    fn vowel_word_adds_hay() {
        let vowel_word = Some(String::from("apple"));
        assert_eq!(pig_latin(vowel_word), Some(String::from("apple-hay")));
    }

    #[test]
    fn non_ascii_word_returns_none() {
        let non_ascii_word = Some(String::from("สวัสดี"));
        assert_eq!(pig_latin(non_ascii_word), None);
    }

    #[test]
    fn none_returns_none() {
        assert_eq!(pig_latin(None), None);
    }

    #[test]
    fn uppercase_returns_lowercase() {
        let uppercase_word = Some(String::from("APPLE"));
        assert_eq!(pig_latin(uppercase_word), Some(String::from("apple-hay")));
    }
}
