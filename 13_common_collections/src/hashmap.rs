pub enum LearnHashMap {
    BasicHashMap,
}

pub fn learn_hash_map(learn: LearnHashMap) -> Option<String> {
    match learn {
        LearnHashMap::BasicHashMap => {
            return basic_hash_map();
        }
    }
}

pub fn basic_hash_map() -> Option<String> {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get the initial score using a key
    let team_name = String::from("Blue");
    let initial_score = scores.get(&team_name).copied().unwrap_or(0);

    // Overwrite an existing value
    let team_to_update = String::from("Blue");
    let new_score = 60;

    scores.insert(team_to_update, new_score);

    // team_to_update was moved into the HashMap
    // new_score is still usable because i32 implements Copy

    let updated_score = scores.get(&team_name).copied().unwrap_or(0);

    // Insert a value only if the key does not exist
    scores.entry(String::from("Yellow")).or_insert(70);
    scores.entry(String::from("Red")).or_insert(20);

    // Update values based on their existing values
    let team_names = "Green Orange Blue Gray";

    for team in team_names.split_whitespace() {
        let score = scores.entry(team.to_string()).or_insert(0);
        *score += 1;
    }

    // Read all key-value pairs
    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    let final_score = scores.get(&team_name).copied().unwrap_or(0);

    Some(format!(
        "Blue score: {initial_score} -> {updated_score} -> {final_score}"
    ))
}

#[cfg(test)]
mod tests_hash_map {
    use super::*;

    #[test]
    fn learn_basic_hash_map() {
        let result = learn_hash_map(LearnHashMap::BasicHashMap);

        assert_eq!(result, Some(String::from("Blue score: 10 -> 60 -> 61")));
    }
}
