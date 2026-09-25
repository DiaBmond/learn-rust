mod string;
mod vector;

use crate::string::*;
use crate::vector::*;

pub enum LearnVector {
    BasicVector,
    ReadingVector,
    OwnerVector,
    IteratingVector,
    EnumVector,
}

pub enum LearnString {
    BasicString,
}

pub fn learn_vector(learn: LearnVector) -> Option<String> {
    match learn {
        LearnVector::BasicVector => {
            let number = 7;
            return basic_vector(Some(number));
        }
        LearnVector::ReadingVector => {
            return reading_vectors();
        }
        LearnVector::OwnerVector => {
            return owner_with_vector();
        }
        LearnVector::IteratingVector => {
            return iterating_over_vector();
        }
        LearnVector::EnumVector => {
            return enum_with_vector();
        }
    }
}

pub fn learn_string(learn: LearnString) -> Option<String> {
    match learn {
        LearnString::BasicString => {
            return basic_string();
        }
    }
}
#[cfg(test)]
mod tests_vector {
    use super::*;

    #[test]
    fn learn_basic_vector() {
        let result = learn_vector(LearnVector::BasicVector);

        assert_eq!(result, Some(String::from("The first element is 7")));
    }

    #[test]
    fn learn_reading_vector() {
        let result = learn_vector(LearnVector::ReadingVector);

        assert_eq!(result, Some(String::from("There is no hundredth element.")));
    }

    #[test]
    fn learn_owner_vector() {
        let result = learn_vector(LearnVector::OwnerVector);

        assert_eq!(result, Some(String::from("Vector length is 6")));
    }

    #[test]
    fn learn_iterating_over_vector() {
        let result = learn_vector(LearnVector::IteratingVector);

        assert_eq!(
            result,
            Some(String::from(
                "Values in v1: [100, 32, 57] Values in v2: [150, 82, 107]"
            ))
        );
    }

    #[test]
    fn learn_enum_with_vector() {
        let result = learn_vector(LearnVector::EnumVector);

        assert_eq!(result, Some(String::from("Int:3 Text:blue Float:10.12 ")));
    }
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
