mod calendar_2024;
mod helper;

use calendar_2024::dec_1;

fn main() {
    println!(
        "{:?}",
        dec_1::find_similarity("./src/calendar_2024/dec_1_input.txt")
    );
}
