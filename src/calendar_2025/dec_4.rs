use crate::calendar_2025::templates::Proj;
use regex::Regex;
use std::fs;

struct Paper {
    file_location: String,
    total: u16,
}

impl Proj<Vec<char>, u16> for Paper {
    fn parse_data(&self, extension: String) -> Vec<Vec<char>> {
        let full_path = format!("{}{}", self.file_location, extension);
        let text = fs::read_to_string(full_path).expect("Error reading file");
        let mut result: Vec<Vec<char>> = vec![vec![]];
        let mut col = 0;
        text.chars().for_each(|c| match c {
            '\r' => {
                result.push(vec![]);
                col += 1;
            }
            '\n' => {}
            _ => result[col].push(c),
        });

        result
    }

    fn run_part1(&mut self, extension: String) -> u16 {
        self.total = 0;
        let result = self.parse_data(extension);
        let row_length = result[0].len();
        let col_length = result.len();

        for row in 0..row_length {
            for col in 0..col_length {
                if result[row][col] != '.' {
                    let c = Self::count_around(&result, row, col);
                    if c < 4 {
                        self.total += 1;
                        print!("x");
                    } else {
                        print!("@");
                    }
                } else {
                    print!(".");
                }
            }
            println!()
        }

        self.total
    }

    fn run_part2(&mut self, extension: String) -> u16 {
        self.total = 0;
        let mut result = self.parse_data(extension);
        let row_length = result[0].len();
        let col_length = result.len();
        let mut pass_total = 0;

        while pass_total != 0 || self.total == 0 {
            pass_total = 0;
            let current_pass = result.clone();
            for row in 0..row_length {
                for col in 0..col_length {
                    if result[row][col] != '.' {
                        let c = Self::count_around(&current_pass, row, col);
                        if c < 4 {
                            pass_total += 1;
                            result[row][col] = '.';
                            // print!("x");
                        } else {
                            // print!("@");
                        }
                    } else {
                        // print!(".");
                    }
                }
                // println!()
            }

            // println!("pass total: {}", pass_total);
            self.total += pass_total;
        }

        self.total
    }
}

impl Paper {
    fn count_around(plane: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
        let mut count = 0;

        let left_valid = col != 0;
        let right_valid = col != plane.first().unwrap().len() - 1;
        let up_valid = row != 0;
        let down_valid = row != plane.len() - 1;

        if up_valid {
            if left_valid {
                //LT
                count += Paper::check(plane, row - 1, col - 1);
            }
            //LM
            count += Paper::check(plane, row - 1, col);
            if right_valid {
                //LR
                count += Paper::check(plane, row - 1, col + 1);
            }
        }

        if left_valid {
            //LM
            count += Paper::check(plane, row, col - 1);
        }

        if right_valid {
            //RM
            count += Paper::check(plane, row, col + 1);
        }

        if down_valid {
            if left_valid {
                //LB
                count += Paper::check(plane, row + 1, col - 1);
            }
            //LB
            count += Paper::check(plane, row + 1, col);
            if right_valid {
                //RB
                count += Paper::check(plane, row + 1, col + 1);
            }
        }

        count as u32
    }

    fn check(plane: &Vec<Vec<char>>, row: usize, col: usize) -> u8 {
        let test = plane[row][col];
        if test == '@' {
            return 1;
        }
        0
    }
}

pub fn test() {
    let mut toilet = Paper {
        file_location: String::from("src/calendar_2025/dec_4/"),
        total: 0,
    };

    if toilet.validate(43, 2) {
        println!("---------SUCCESS----------");
        println!("{}", toilet.run_part2("input".to_string()));
    } else {
        println!("{}", toilet.run_part2("sample".to_string()));
        println!("---------ERROR----------");
    }
}
