mod calendar_2024 {
    pub mod dec_2;
}

use crate::calendar_2024::dec_2;

mod helper;

fn main() {
    println!(
        "{}",
        dec_2::record_safety("src/calendar_2024/dec_2_txt/dec_2_input.txt")
    );
}
