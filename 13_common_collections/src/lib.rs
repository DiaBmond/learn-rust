mod vector;

use crate::vector::*;

pub enum Learn {
    BasicVector,
    ReadingVector,
    OwnerVector,
    IteratingVector,
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
        Learn::OwnerVector => {
            return owner_with_vector();
        }
        Learn::IteratingVector => {
            return iterating_over_vector();
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

    #[test]
    fn learn_owner_vector() {
        let result = learn_vector(Learn::OwnerVector);

        assert_eq!(result, Some(String::from("Vector length is 6")));
    }

    #[test]
    fn learn_iterating_over_vector() {
        let result = learn_vector(Learn::IteratingVector);

        assert_eq!(
            result,
            Some(String::from(
                "Values in v1: [100, 32, 57] Values in v2: [150, 82, 107]"
            ))
        );
    }
}
