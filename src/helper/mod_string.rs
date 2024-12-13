pub fn count_col(const_arr: &String) -> i32 {
    for (i, c) in const_arr.chars().enumerate() {
        if '\n' == c {
            return i as i32;
        }
    }

    return 0;
}

pub fn pad_array(const_arr: String, col: i32) -> String {
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
