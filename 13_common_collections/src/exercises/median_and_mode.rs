pub fn median_and_mode(numbers: Option<Vec<i32>>) -> (Option<f64>, Option<Vec<i32>>) {
    let Some(mut numbers) = numbers else {
        return (None, None);
    };

    if numbers.is_empty() {
        return (None, None);
    }

    let median_result = median(&mut numbers);
    let mode_result = modes(&numbers);

    (median_result, mode_result)
}

fn median(numbers: &mut Vec<i32>) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    numbers.sort();
    if numbers.len() % 2 == 0 {
        let middle_sum = numbers[numbers.len() / 2] + numbers[(numbers.len() / 2) - 1];
        let median = middle_sum as f64 / 2.0;
        return Some(median);
    } else {
        return Some(numbers[numbers.len() / 2] as f64);
    }
}

use std::collections::HashMap;

fn modes(numbers: &Vec<i32>) -> Option<Vec<i32>> {
    let mut counts = HashMap::new();

    for &number in numbers {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    let Some(&max_count) = counts.values().max() else {
        return None;
    };

    if max_count == 1 {
        return None;
    }

    let mut modes = Vec::new();

    for (number, count) in counts {
        if count == max_count {
            modes.push(number);
        }
    }

    modes.sort();

    Some(modes)
}

#[cfg(test)]
mod unit_tests_exercises_1 {
    use super::*;

    mod median {
        use super::*;

        #[test]
        fn empty_vec_returns_none() {
            let mut empty_input = vec![];
            assert_eq!(median(&mut empty_input), None);
        }

        #[test]
        fn even_length_returns_average_of_middle_two() {
            let mut even_length_input = vec![4, 1, 3, 2];
            assert_eq!(median(&mut even_length_input), Some(2.5));
        }

        #[test]
        fn odd_length_returns_middle() {
            let mut odd_length_input = vec![2, 3, 1, 7, 5, 9, 10];
            assert_eq!(median(&mut odd_length_input), Some(5.0));
        }

        #[test]
        fn negative_numbers_handled_correctly() {
            let mut negative_input = vec![-2, -3, 1, -7, 5];
            assert_eq!(median(&mut negative_input), Some(-2.0));
        }
    }

    mod modes {
        use super::*;

        #[test]
        fn empty_vec_returns_none() {
            let empty_input = vec![];
            assert_eq!(modes(&empty_input), None);
        }

        #[test]
        fn single_mode_returns_one_value() {
            let single_mode = vec![4, 1, 1, 3, 2];
            assert_eq!(modes(&single_mode), Some(vec![1]));
        }

        #[test]
        fn two_modes_return_two_values() {
            let two_modes = vec![4, 1, 1, 3, 3, 2];
            assert_eq!(modes(&two_modes), Some(vec![1, 3]));
        }

        #[test]
        fn all_unique_values_return_none() {
            let unique_values = vec![4, 1, 3, 2];
            assert_eq!(modes(&unique_values), None);
        }

        #[test]
        fn single_element_returns_none() {
            let single_element = vec![5];
            assert_eq!(modes(&single_element), None);
        }

        #[test]
        fn all_identical_elements_return_that_element() {
            let all_same = vec![7, 7, 7, 7];
            assert_eq!(modes(&all_same), Some(vec![7]));
        }
    }
}
