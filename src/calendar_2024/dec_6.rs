use std::{fs, ops::Index};

use crate::helper::mod_string::{count_col, pad_array};

struct Direction {
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

pub fn count_steps(filepath: &str) -> i32 {
    let content = fs::read_to_string(filepath)
        .expect("Could not read file")
        .to_string();

    let col = count_col(&content);
    let temp_arr = pad_array(content, col + 2);
    let mut pad = temp_arr.chars();
    let mut index: usize = 0;

    for (i, c) in pad.clone().enumerate() {
        if c.to_string() == "^" {
            index = i;
            break;
        }
    }

    println!("{}", index);

    let mut c = pad.nth(index).unwrap();
    while c.to_string() != " " {
        println!("{}", c);
        index += 1;
        println!("{}", pad.nth(index).unwrap());
        let bind = pad.nth(index);
        if bind == None {
            break;
        } else {
            c = bind.unwrap();
        }

        println!("{}", c);
    }

    5
}
