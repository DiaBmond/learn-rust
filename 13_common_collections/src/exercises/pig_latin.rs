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
