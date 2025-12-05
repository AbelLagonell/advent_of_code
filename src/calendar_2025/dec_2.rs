use regex::Regex;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

struct Tally {
    running_total: u64,
    file_location: String,
}

struct Set {
    start: u64,
    end: u64,
}

impl Tally {
    fn parse_data(&self, extension: String) -> Vec<Set> {
        let full_path = format!("{}{}", self.file_location, extension);
        let hay = fs::read_to_string(full_path).expect("Error reading file");
        let re = Regex::new(r"([0-9]+)-([0-9]+)").unwrap();
        let mut results = vec![];
        for (_, [start, end]) in re.captures_iter(hay.as_str()).map(|c| c.extract()) {
            results.push(Set {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            });
        }
        results
    }

    fn run(&mut self, extension: String) -> u64 {
        let results = self.parse_data(extension);
        for set in results {
            for i in set.start..(set.end + 1) {
                let base: u64 = 10;
                let power = i.to_string().len() as u32;
                if power % 2 == 1 {
                    continue;
                }
                let divisor = base.pow(power / 2) + 1;
                let number = i % divisor;
                if number == 0 {
                    self.running_total += i;
                }
            }
        }

        self.running_total
    }

    fn run2(&mut self, extension: String) -> u64 {
        let results = self.parse_data(extension);
        for set in results {
            for i in set.start..(set.end + 1) {
                let n = i.to_string();
                let mut r = 1;
                let d = n.len();
                while d/r >= 2{
                    let tp = n[0..r].to_string();
                    let tp2 = n[n.len()-r..n.len()].to_string();
                    // println!("with number: {} is {} == {}? {}", i, tp, tp2, tp==tp2);
                    if tp != tp2{
                        r += 1;
                        continue;
                    }
                    let divisor = Self::build_divisor(r-1, d/r-1);
                    if i % divisor == 0{
                        self.running_total += i;
                        println!("Pattern Number {} with {}", i, divisor);
                        break;
                    }
                    r+= 1;
                }
            }
        }

        self.running_total
    }

    fn build_divisor(z:usize, i:usize)-> u64{
        let mut string: String = "".to_string();
        for _ in 0..i{
            string += "1";
            for _ in 0..z{
                string += "0";
            }
        }
        string += "1";
        // println!("\t{}", string);
        string.parse().unwrap()
    }

    fn validate(&mut self, valid: u64) -> bool {
        self.run2("sample".to_string());
        self.running_total == valid
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}
impl Debug for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

pub fn test() {
    let mut tally = Tally {
        running_total: 0,
        file_location: String::from("src/calendar_2025/dec_2/"),
    };

    if tally.validate(4174379265) {
        println!("--------SUCCESS----------");
        tally.running_total = 0;
        // println!("{}", tally.run2("input".to_string()));
    } else {
        println!("--------ERROR----------\n{}", tally.running_total);
    }
}
