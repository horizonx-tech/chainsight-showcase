pub fn date_to_epoch(year: u32, month: u32, day: u32) -> i64 {
    let mut days = 0;

    // Add days from 1970 to "year"
    for y in 1970..year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }

    // Add days from January to one month before "month".
    let month_days = [31, if is_leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];    
    for m in 0..(month - 1) {
        days += month_days[m as usize];
    }

    // Add days from "day"
    days += day;

    // epoch seconds (from 1970.1.1)
    (days as i64 - 1) * 86400
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}