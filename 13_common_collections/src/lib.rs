mod vector;

use crate::vector::*;

pub enum Learn {
    BasicVector,
    ReadingVector,
}

pub fn learn_vector(learn: Learn) -> Option<String> {
    match learn {
        Learn::BasicVector => {
            let number = 7;
            return basic_vector(Some(number));
        }
        Learn::ReadingVector => {
            return reading_vectors();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn learn_basic_vector() {
        let result = learn_vector(Learn::BasicVector);

        assert_eq!(result, Some(String::from("The first element is 7")));
    }

    #[test]
    fn learn_reading_vector() {
        let result = learn_vector(Learn::ReadingVector);

        assert_eq!(result, Some(String::from("There is no hundredth element.")));
    }
}
