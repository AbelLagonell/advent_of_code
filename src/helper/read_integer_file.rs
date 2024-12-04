use std::fs;

const VECFALSE: usize = 10000;

pub fn read_file_columns(filepath: &str) -> Vec<Vec<i32>> {
    let mut parsed = Vec::<Vec<i32>>::new();

    let mut _contents = fs::read_to_string(filepath)
        .expect("Should have been able to read file")
        .to_string();

    let mut first = 0;
    let mut vec_index: usize = VECFALSE;
    for (i, c) in _contents.chars().enumerate() {
        if c.is_numeric() && !_contents.chars().nth(first).unwrap().is_numeric() {
            first = i;
        } else if !c.is_numeric() && c != _contents.chars().nth(first).unwrap() {
            let mut s = "".to_owned();
            for j in first..i {
                s.push(_contents.chars().nth(j).unwrap());
            }
            let integer: i32 = s.parse().unwrap();

            first = i;

            if vec_index != VECFALSE {
                parsed[vec_index].push(integer);
                vec_index = (vec_index + 1) % parsed.len();
            } else {
                let mut new_vec = Vec::<i32>::new();
                new_vec.push(integer);
                parsed.push(new_vec);
            }
        }
        if c == '\n' && vec_index == VECFALSE {
            vec_index = 0;
        }
    }

    return parsed;
}

pub fn read_file_rows(filepath: &str) -> Vec<Vec<i32>> {
    let mut parsed = Vec::<Vec<i32>>::new();

    let mut _contents = fs::read_to_string(filepath)
        .expect("Should have been able to read file")
        .to_string();

    let mut first = 0;
    let mut vec_index: usize = 0;
    let mut start = true;
    for (i, c) in _contents.chars().enumerate() {
        if c.is_numeric() && !_contents.chars().nth(first).unwrap().is_numeric() {
            first = i;
        } else if !c.is_numeric() && c != _contents.chars().nth(first).unwrap() {
            let mut s = "".to_owned();
            for j in first..i {
                s.push(_contents.chars().nth(j).unwrap());
            }
            let integer: i32 = s.parse().unwrap();

            first = i;

            if start {
                let mut new_vec = Vec::<i32>::new();
                new_vec.push(integer);
                parsed.push(new_vec);
                start = false;
            } else {
                parsed[vec_index].push(integer);
            }
        }

        if c == '\n' {
            vec_index += 1;
            start = true;
        }
    }

    return parsed;
}
