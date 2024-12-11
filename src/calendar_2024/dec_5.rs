use regex::Regex;
use std::{
    fs,
    ops::{ControlFlow, Index},
};

use crate::helper::read_integer_file::read_file_rows;

pub fn print_queue(filepath: &str) -> i32 {
    let content = fs::read_to_string(filepath)
        .expect("Not able to read file")
        .to_string();

    let (rules, lists) = seperate_content(&content);

    let rules_dictionary = parse_rules(&rules);
    let test_vecs = read_file_rows(&lists);
    let mut passed_lists: Vec<bool> = vec![];

    for vec in &test_vecs {
        let mut local_bool = true;

        //Iterates through one list
        for index in 0..vec.len() {
            let value = vec[index];
            //Checks for rule that is going to be needed
            if !&rules_dictionary.iter().any(|(key, _)| *key == value) {
                continue;
            }

            for (key, def) in &rules_dictionary {
                if *key != value {
                    continue;
                }

                for n_index in 0..index {
                    if def.contains(&vec[n_index]) {
                        local_bool = false;
                    }
                }
            }
        }

        passed_lists.push(local_bool);
    }

    let mut sum = 0;
    for i in 0..passed_lists.len() {
        if passed_lists[i] {
            let vec_len = &test_vecs[i].len();
            sum += test_vecs[i][(*vec_len as f32 / 2.0).floor() as usize];
        }
    }

    sum
}

pub fn print_incorrect_queue(filepath: &str) -> i32 {
    let content = fs::read_to_string(filepath)
        .expect("Not able to read file")
        .to_string();

    let (rules, lists) = seperate_content(&content);

    let rules_dictionary = parse_rules(&rules);
    let mut test_vecs = read_file_rows(&lists);
    let mut passed_lists: Vec<bool> = vec![];

    for vec in &test_vecs {
        let mut local_bool = true;

        for index in 0..vec.len() {
            let value = vec[index];
            if !&rules_dictionary.iter().any(|(key, _)| *key == value) {
                continue;
            }

            for (key, def) in &rules_dictionary {
                if *key != value {
                    continue;
                }

                for n_index in 0..index {
                    if def.contains(&vec[n_index]) {
                        local_bool = false;
                    }
                }
            }
        }
        passed_lists.push(local_bool);
    }

    let mut sum = 0;
    // Correct the Wrong ones
    for index in 0..passed_lists.len() {
        if passed_lists[index] {
            continue;
        }

        let w_list = &mut test_vecs[index];
        let mut c_list: Vec<i32> = vec![w_list[0]];

        for i in 1..w_list.len() {
            let mut index = 0;
            for j in 0..c_list.len() {
                for (key, def) in &rules_dictionary {
                    //Making sure that the c_list key is the correct one
                    if *key != c_list[j] {
                        continue;
                    }

                    if def.contains(&w_list[i]) {
                        index += 1;
                        break;
                    }
                }
            }
            c_list.insert(index, w_list[i]);
        }
        let vec_len = &c_list.len();
        sum += c_list[(*vec_len as f32 / 2.0).floor() as usize];
    }

    sum
}

fn parse_rules(content: &str) -> Vec<(i32, Vec<i32>)> {
    let mut rules: Vec<(i32, Vec<i32>)> = vec![];

    let re = Regex::new(r"([0-9]+)\|([0-9]+)");

    for (_, [i, j]) in re.expect("").captures_iter(&content).map(|c| c.extract()) {
        let x: i32 = i.parse().unwrap();
        let y: i32 = j.parse().unwrap();

        if rules.iter().any(|(key, _)| *key == x) {
            for (key, value) in &mut rules {
                if *key == x {
                    value.push(y);
                    value.sort();
                }
            }
        } else {
            rules.push((x, vec![y]));
        }
    }

    return rules;
}

fn seperate_content(content: &str) -> (String, String) {
    let mut different: bool = false;
    let mut first: String = "".to_string();
    let mut second: String = "".to_string();
    for (i, c) in content.chars().enumerate() {
        if different {
            second += &c.to_string();
        } else {
            first += &c.to_string();
        }
        if '\n' == c && '\n' == content.chars().nth(i - 1).unwrap() {
            different = true;
        }
    }

    return (first, second);
}
