mod calendar_2024 {
    pub mod dec_6;
}

mod helper;

use crate::calendar_2024::dec_6;

const FILE_PATH: &str = "src/calendar_2024/dec_6/";

fn main() {
    println!(
        "{:?}",
        dec_6::count_steps(&(FILE_PATH.to_owned() + "sample.txt"))
    );
}
