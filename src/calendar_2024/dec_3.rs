use regex::Regex;
use std::fs;

pub fn find_mul(filepath: &str) -> i32 {
    let content = fs::read_to_string(filepath)
        .expect("Should have been able to read file")
        .to_string();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)");

    let mut sum = 0;
    for (_, [i, j]) in re.expect("").captures_iter(&content).map(|c| c.extract()) {
        let n: i32 = i.parse().unwrap();
        let m: i32 = j.parse().unwrap();
        sum += n * m;
    }

    sum
}

pub fn find_cond(filepath: &str) -> i32 {
    let content = fs::read_to_string(filepath)
        .expect("Should have been able to read file")
        .to_string();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(don't)()|(do)()");

    let mut add = true;
    let mut sum = 0;

    for cap in re.expect("").captures_iter(&content) {
        let full_result: (&str, [&str; 2]) = cap.extract();
        let first = full_result.1.first().unwrap();
        if "do" == *first {
            add = true;
            continue;
        }
        if "don't" == *first {
            add = false;
            continue;
        }
        let n: i32 = first.parse().unwrap();
        let m: i32 = full_result.1.last().unwrap().parse().unwrap();

        println!("{}, {}", n, m);

        if add {
            sum += n * m
        }
    }

    sum
}
