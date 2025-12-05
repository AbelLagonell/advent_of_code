pub trait Proj<I,O: PartialEq>{
    fn parse_data(&self, extension: String) -> Vec<I>;
    fn run_part1(&mut self, extension: String) -> O;
    fn run_part2(&mut self, extension: String) -> O;
    
    fn validate(&mut self, valid: O, part: u8) -> bool{
        match part { 
            1 => self.run_part1("sample".to_string()) == valid,
            2 => self.run_part2("sample".to_string()) == valid,
            _ => false 
        }
    }
}