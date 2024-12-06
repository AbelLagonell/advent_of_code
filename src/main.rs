mod calendar_2024 {
    pub mod dec_3;
}

use crate::calendar_2024::dec_3;

//mod helper;

const FILE_PATH: &str = "src/calendar_2024/dec_3_txt/";

fn main() {
    println!(
        "{}",
        dec_3::find_cond(&(FILE_PATH.to_owned() + "dec_3_input.txt"))
    );
}
