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
