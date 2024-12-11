use std::{char, fs};

const XMAS: &str = "XMAS";

pub fn convert_to_array(filepath: &str) -> String {
    fs::read_to_string(filepath)
        .expect("Should have been able to read file")
        .to_string()
}

pub fn find_xmas(filepath: &str) -> i32 {
    let _content = convert_to_array(filepath);
    let col = count_col(&_content);
    let content = pad_array(_content, col);

    let mut count = 0;

    for (i, c) in content.chars().enumerate() {
        if " " == c.to_string() {
            continue;
        } else if "X" == c.to_string() {
            for dir in [-col - 1, -col, 1 - col, -1, 1, col - 1, col, 1 + col] {
                if XMAS.to_string() == line_search(content.clone(), i as i32, dir, "X".to_string())
                {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn find_x_mas(filepath: &str) -> i32 {
    let _content = convert_to_array(filepath);
    let col = count_col(&_content);
    let content = pad_array(_content, col);

    let mut count = 0;

    for (i, c) in content.chars().enumerate() {
        if " " == c.to_string() {
            continue;
        }
        if "A" == c.to_string() {
            let content_copy = content.clone();
            count += x_search(content_copy, i as i32, col);
        }
    }

    count
}

fn x_search(const_arr: String, index: i32, cols: i32) -> i32 {
    let positions = [
        index + cols - 1, // Bot Left
        index + cols + 1, // Bot Right
        index - cols - 1, // Top Left
        index - cols + 1, // Top Right
    ];

    let chars = [
        const_arr.chars().nth(positions[0] as usize),
        const_arr.chars().nth(positions[1] as usize),
        const_arr.chars().nth(positions[2] as usize),
        const_arr.chars().nth(positions[3] as usize),
    ];

    if chars.contains(&None) {
        return 0;
    }

    let test = chars.map(|c| c.unwrap());

    if test.contains(&' ') || test.contains(&'X') || test.contains(&'A') {
        return 0;
    }

    if test[0] == test[1] && test[2] == test[3] && test[0] != test[2] {
        println!("Vertical: {},{}", index % cols, index / cols);
        return 1;
    }

    if test[0] == test[2] && test[1] == test[3] && test[0] != test[1] {
        println!("Horizontal: {},{}", index % cols, index / cols);
        return 1;
    }

    return 0;
}

fn line_search(const_arr: String, index: i32, const_dir: i32, cur_str: String) -> String {
    if cur_str == "" {
        return "".to_string();
    }

    let c_char = const_arr.chars().nth((index + const_dir) as usize);
    if None == c_char {
        return "".to_string();
    }

    if c_char.unwrap() == ' ' {
        return "".to_string();
    }

    if cur_str.chars().nth(cur_str.len() - 1).unwrap() == c_char.unwrap() {
        return "".to_string();
    }

    let n_str = cur_str + &c_char.unwrap().to_string();
    if n_str.len() > XMAS.len() {
        return "".to_string();
    }
    if n_str == XMAS {
        return n_str;
    }

    return line_search(const_arr, index + const_dir, const_dir, n_str);
}

fn pad_array(const_arr: String, col: i32) -> String {
    let mut content: String = "".to_string();
    for _ in 0..col {
        content += " ";
    }
    for (i, c) in const_arr.chars().enumerate() {
        if i as i32 % (col - 1) == 0 {
            content += " ";
        }

        if '\n' == c {
            content += " ";
            continue;
        }
        content += &c.to_string();
    }

    for _ in 0..col {
        content += " ";
    }

    content
}

fn count_col(const_arr: &String) -> i32 {
    for (i, c) in const_arr.chars().enumerate() {
        if '\n' == c {
            return i as i32 + 2;
        }
    }

    return 0;
}
