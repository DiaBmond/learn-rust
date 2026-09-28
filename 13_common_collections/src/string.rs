pub enum LearnString {
    BasicString,
    UpdateString,
    IndexingString,
}

pub fn learn_string(learn: LearnString) -> Option<String> {
    match learn {
        LearnString::BasicString => {
            return basic_string();
        }
        LearnString::UpdateString => {
            return update_string();
        }
        LearnString::IndexingString => {
            return indexing_string();
        }
    }
}

pub fn basic_string() -> Option<String> {
    let empty = String::new();

    let data = "initial contents";
    let from_variable = data.to_string();

    let from_literal = "initial contents".to_string();

    let from_function = String::from("initial contents");

    let thai = String::from("สวัสดี");
    let japanese = String::from("こんにちは");
    let chinese = String::from("你好");

    Some(format!(
        "Empty: '{empty}', Variable: '{from_variable}', Literal: '{from_literal}', From: '{from_function}', Thai: '{thai}', Japanese: '{japanese}', Chinese: '{chinese}'"
    ))
}

pub fn update_string() -> Option<String> {
    let mut s1 = String::from("Learning ");
    let s2 = "Rust is fu";
    s1.push_str(s2);

    s1.push('n');

    let s3 = String::from(" and useful");
    let s4 = s1 + &s3;

    let s5 = String::from("Keep practicing");
    let s6 = String::from("Keep learning");

    Some(format!("{s4} - {s5} - {s6}"))
}

pub fn indexing_string() -> Option<String> {
    let hello = "Здравствуйте";

    let slice = &hello[0..4];
    // let slice = &hello[0..1]; // panic

    let mut chars = String::from("Chars: ");
    for c in "Зд".chars() {
        chars.push(c);
    }

    let mut bytes = String::from("Bytes:");

    for b in "Зд".bytes() {
        bytes.push_str(&format!(" {b}"));
    }

    Some(format!("Slice: {slice} | {chars} | {bytes}"))
}

#[cfg(test)]
mod tests_string {
    use super::*;

    #[test]
    fn learn_basic_string() {
        let result = learn_string(LearnString::BasicString);

        assert_eq!(
            result,
            Some(String::from(
                "Empty: '', Variable: 'initial contents', Literal: 'initial contents', From: 'initial contents', Thai: 'สวัสดี', Japanese: 'こんにちは', Chinese: '你好'"
            ))
        );
    }

    #[test]
    fn learn_update_string() {
        let result = learn_string(LearnString::UpdateString);

        assert_eq!(
            result,
            Some(String::from(
                "Learning Rust is fun and useful - Keep practicing - Keep learning"
            ))
        );
    }

    #[test]
    fn learn_indexing_string() {
        let result = learn_string(LearnString::IndexingString);

        assert_eq!(
            result,
            Some(String::from(
                "Slice: Зд | Chars: Зд | Bytes: 208 151 208 180"
            ))
        );
    }
}
