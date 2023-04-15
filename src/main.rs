use std::str::FromStr;
use std::env;

fn main() {
    let start_str = match env::args().nth(1) {
        Some(s) => s,
        None => String::from("0"),
    };
    let stop_str = match env::args().nth(2) {
        Some(s) => s,
        None => String::from("10"),
    };

    let start: i32 = match i32::from_str(&start_str) {
        Ok(i) => i,
        _ => 0,
    };
    let stop: i32 = match i32::from_str(&stop_str) {
        Ok(i) => i,
        _ => 10,
    };

    for i in start..=stop {
        println!("{i}");
    };
}
