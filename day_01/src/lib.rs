use std::collections::HashSet;

pub fn sum_values(starting_value: Option<i64>, values_list: &Vec<i64>) -> i64 {
    let mut current_value : i64 = starting_value.unwrap_or(0);
    for value in values_list.iter() { current_value = current_value + value }

    return current_value;
}

pub fn first_repeated_current(starting_value: Option<i64>, values_list: &Vec<i64>) -> i64 {
    let mut current_value : i64 = starting_value.unwrap_or(0);
    let mut hash_set : HashSet<i64> = HashSet::new();

    let mut found_rep : bool = false;

    while !found_rep {
        for value in values_list.iter() {

            current_value = current_value + value;
            if hash_set.contains(&current_value) {
                found_rep = true;
                break;
            }
            hash_set.insert(current_value);
        }
    }

    return current_value;
}