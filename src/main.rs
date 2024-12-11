mod calendar_2024 {
    pub mod dec_5;
}

mod helper {
    pub mod read_integer_file;
}

use crate::calendar_2024::dec_5;

const FILE_PATH: &str = "src/calendar_2024/dec_5/";

fn main() {
    println!(
        "{:?}",
        dec_5::print_incorrect_queue(&(FILE_PATH.to_owned() + "input.txt"))
    );
}
