use regex::Regex;
use std::fs;

pub struct Dial {
    pub current_dial: i16,
    pub count_of_zeroes: u16,
    pub clicks_of_zeroes: u16,
    pub file_location: String,
}

impl Dial {
    fn right_spin(&mut self, n: i16) {
        self.current_dial += n;
        while self.current_dial >= 100 {
            self.current_dial -= 100;
            self.clicks_of_zeroes += 1;
        }

        if self.current_dial == 0 {
            self.count_of_zeroes += 1;
        }
    }

    fn left_spin(&mut self, n: i16) {
        self.current_dial -= n;
        while self.current_dial < 0{
            self.current_dial += 100;
            self.clicks_of_zeroes += 1;
        }

        if self.current_dial == 0 {
            self.count_of_zeroes += 1;
        }
    }

    fn parse_data(&self, extension: String) -> Vec<String> {
        let full_path = format!("{}{}", self.file_location, extension);
        let hay = fs::read_to_string(full_path).expect("Error reading file");
        let re = Regex::new(r"([L|R][0-9]+)").unwrap();
        let mut results = vec![];
        for (_, [value]) in re.captures_iter(hay.as_str()).map(|c| c.extract()) {
            results.push(value.to_string());
        }
        results
    }
    pub fn run(&mut self, extension: String) -> (u16,u16) {
        let results = self.parse_data(extension);
        for operation in results {
            let result= operation[1..operation.len()].parse();
            if result.is_err(){
                eprintln!("{}", result.unwrap_err());
                return (0,0);
            }

            let number = result.unwrap();

            match operation[0..1].to_string().as_str() {
                "L" => self.left_spin(number),
                "R" => self.right_spin(number),
                _ => {}
            }
        }
        (self.count_of_zeroes, self.clicks_of_zeroes)
    }

    pub fn validate(&mut self, valid: (u16, u16)) -> bool {
        self.run("sample".to_string());
        self.count_of_zeroes == valid.0 && self.clicks_of_zeroes == valid.1
    }
}

pub fn test() {
    let mut dial = Dial {
        current_dial: 50,
        count_of_zeroes: 0,
        clicks_of_zeroes: 0,
        file_location: String::from("src/calendar_2025/dec_1/"),
    };

    if dial.validate((3, 6)) {
        dial.count_of_zeroes = 0;
        dial.current_dial = 50;
        dial.clicks_of_zeroes = 0;
        println!("--------SUCCESS---------");
        let results = dial.run("input".to_string());
        println!("{}, {}", results.0, results.1);
    }
}
