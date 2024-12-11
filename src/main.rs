mod calendar_2024 {
    pub mod dec_4;
}

use crate::calendar_2024::dec_4;

//mod helper;

const FILE_PATH: &str = "src/calendar_2024/dec_4/";

fn main() {
    println!(
        "{:?}",
        dec_4::find_x_mas(&(FILE_PATH.to_owned() + "input.txt"))
    );
}
