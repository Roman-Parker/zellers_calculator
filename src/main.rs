// This is a simple calculator that calculates the day of the week for any given date in the gregorian calendar.
// The calculation im using Zellers Congruence

// Zellers formula
// h = (q + [13(m+1)/5] + K + [K/4] + [J/4] - 2J) mod 7

use std::collections::HashMap;
use text_io::read;



fn main() {
    let weekday_map = HashMap::from([
        (0, "Saturday"),
        (1, "Sunday"),
        (2, "Monday"),
        (3, "Tuesday"),
        (4, "Wednesday"),
        (5, "Thursday"),
        (6, "Friday"),
    ]);

    let mut h: i32;
    let q: i32 = 1;
    let user_date = get_user_input();
}



// fn adjust_for_leap_year() {

// }

fn get_user_input() -> String {
    println!("Enter date");
    let mut input: String = read!();
    input
}