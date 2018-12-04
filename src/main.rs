use std::env;

mod day_1;
mod day_2;
mod day_3;
pub mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let err_msg = "Enter a number between 1 - 24.";
    let day: u8 = args[1].trim().parse().expect(err_msg);

    lib::clear();

    match day {
        1 => day_1::solve(),
        // 2 => day_2::solve(),
        3 => day_3::solve(),
        // 4 => day_4::solve(),
        // 5 => day_5::solve(),
        // 6 => day_6::solve(),
        // 7 => day_7::solve(),
        // 8 => day_8::solve(),
        // 9 => day_9::solve(),
        // 10 => day_10::solve(),
        // 11 => day_11::solve(),
        // 12 => day_12::solve(),
        // 13 => day_13::solve(),
        // 14 => day_14::solve(),
        // 15 => day_15::solve(),
        // 16 => day_16::solve(),
        // 17 => day_17::solve(),
        // 18 => day_18::solve(),
        // 19 => day_19::solve(),
        // 20 => day_20::solve(),
        // 21 => day_21::solve(),
        // 22 => day_22::solve(),
        // 23 => day_23::solve(),
        // 24 => day_24::solve(),
        _ => println!("{}", err_msg),
    }
}
