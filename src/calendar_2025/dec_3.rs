use crate::calendar_2025::templates::Proj;
use regex::Regex;
use std::fs;

struct Joltages {
    file_location: String,
    running_total: u64,
}

impl Proj<String, u64> for Joltages {
    fn parse_data(&self, extension: String) -> Vec<String> {
        let full_path = format!("{}{}", self.file_location, extension);
        let hay = fs::read_to_string(full_path).expect("Error reading file");
        let re = Regex::new(r"([0-9]+)").unwrap();
        let mut results = vec![];
        for (_, [value]) in re.captures_iter(hay.as_str()).map(|c| c.extract()) {
            results.push(value.to_string());
        }
        results
    }

    fn run_part1(&mut self, extension: String) -> u64 {
        self.running_total = 0;
        let results = self.parse_data(extension);
        for number in results {
            let mut first = 0;
            let mut second = 0;
            let mut chars = number.chars();
            let mut c = chars.next();
            while c != None {
                let n = c.unwrap().to_digit(10).unwrap() as u64;
                if n > second {
                    second = n;
                }
                if n > first && chars.clone().count() >= 1 {
                    first = n;
                    second = 0;
                }
                c = chars.next();
            }

            println!("Number {}", first * 10 + second);

            self.running_total += first * 10 + second;
        }

        self.running_total
    }

    fn run_part2(&mut self, extension: String) -> u64 {
        self.running_total = 0;
        let results = self.parse_data(extension);
        for number in results {
            let mut array: [u64; 12] = [0; 12];
            let mut chars = number.chars();
            let mut c = chars.next();
            let mut a = 0;
            while c != None {
                let remainder = chars.clone().count() as u64;
                let n = c.unwrap().to_digit(10).unwrap() as u64;
                let temp = 12-(remainder+1) as i32;
                let mut start = 0;
                if temp >0  && temp < 12 {
                    start = temp;
                }
                let mut found = false;
                for i in start..a+1 {
                    if found {
                        array[i as  usize] = 0;
                        continue;
                    }
                    if n > array[i as usize] {
                        array[i as usize] = n;
                        found = true;
                    }
                } 
                if a < 11 && found{
                    a += 1;
                }
                c = chars.next();
            }
            self.running_total += array.iter().fold(0u64, |acc, &digit| acc*10 + digit);
        }

        self.running_total
    }
}

pub fn test() {
    let mut joltages = Joltages {
        file_location: String::from("src/calendar_2025/dec_3/"),
        running_total: 0,
    };

    if joltages.validate(3121910778619, 2){
        println!("---------SUCCESS----------");
        println!("{}", joltages.run_part2("input".to_string()));
    }
}
