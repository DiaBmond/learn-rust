pub enum LearnString {
    BasicString,
}

pub fn learn_string(learn: LearnString) -> Option<String> {
    match learn {
        LearnString::BasicString => {
            return basic_string();
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
}
