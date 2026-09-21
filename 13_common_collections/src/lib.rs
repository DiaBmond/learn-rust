mod vector;
use crate::vector::*;

enum Learn {
    BasicVector,
    ReadingVector,
}

pub fn learn_vector(learn: Learn) -> Option<T> {
    match learn {
        BasicVector => {
            let number 7 ;
            basic_vector(Some(number));}
        ReadingVector => {
            reading_vectors();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_correct() {
    //     let number = 7;
    //     let result = learn_vector(Some(number));

    //     assert_eq!(result, Some(7));
    // }

    // #[test]
    // fn it_fail() {
    //     let result = learn_vector(None);

    //     assert_eq!(result, None);
    // }
}
