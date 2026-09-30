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
