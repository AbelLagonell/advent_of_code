use std::collections::HashMap;

use crate::helper::read_integer_file;

pub fn find_difference(filepath: &str) -> i32 {
    let mut lists = read_integer_file::read_file_columns(filepath);

    for vec in &mut lists {
        vec.sort();
    }

    let mut sum = 0;
    for index in 0..lists[0].len() {
        sum += (lists[0][index] - lists[1][index]).abs();
    }

    return sum;
}

pub fn find_similarity(filepath: &str) -> i32 {
    let lists = read_integer_file::read_file_columns(filepath);

    let mut reference_list: HashMap<i32, i32> = HashMap::default();
    let mut second_list: HashMap<i32, i32> = HashMap::default();

    for index in 0..lists[0].len() {
        let reference_value = &lists[0][index];
        add_to_hash(&mut reference_list, *reference_value);

        let second_value = &lists[1][index];
        add_to_hash(&mut second_list, *second_value);
    }

    let mut sum = 0;
    for (key, value) in reference_list {
        let second_value: i32;
        let result = second_list.get(&key);
        if result == None {
            second_value = 0
        } else {
            second_value = *result.unwrap();
        }

        sum += key * value * second_value;
    }

    return sum;
}

fn add_to_hash(hash: &mut HashMap<i32, i32>, key: i32) {
    let result = hash.get(&key);
    let val;
    if None == result {
        hash.insert(key, 1);
    } else {
        val = *result.unwrap();
        hash.insert(key, val + 1);
    }
}
