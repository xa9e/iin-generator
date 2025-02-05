use chrono::Datelike;
use chrono::NaiveDate;
use date::dates_between;

mod cli;
mod date;
fn main() {
    cli::run();
}

pub fn gen(start: &str, end: &str, is_male: bool, take_in_a_day: u16) {
    for info in dates_between(start, end) {
        let n7 = n7_from_date(&info.date, is_male);
        for n8_11 in 0..=take_in_a_day {
            let t = format!("{}{}{:04}", info.formatted, n7, n8_11);
            if let Some(n12) = chksum(&t) {
                println!("{t}{n12}");
            }
        }
    }
}

const MULTIPLIERS_SECOND: [u32; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 2];
pub fn chksum(s: &str) -> Option<u32> {
    if s.len() != 11 {
        return None;
    }
    let bytes = s.as_bytes();
    let mut sum = 0u32;
    // Первый проход: веса 1, 2, …, 11
    for (i, &b) in bytes.iter().enumerate() {
        let digit = b.wrapping_sub(b'0');
        if digit > 9 {
            return None;
        }
        sum += (i as u32 + 1) * digit as u32;
    }
    let mut res = sum % 11;
    if res != 10 {
        return Some(res);
    }
    // Второй проход: веса из MULTIPLIERS_SECOND
    sum = 0;
    for (i, &b) in bytes.iter().enumerate() {
        // Уже проверили, что это цифры
        let digit = (b - b'0') as u32;
        sum += MULTIPLIERS_SECOND[i] * digit;
    }
    res = sum % 11;
    if res == 10 {
        None
    } else {
        Some(res)
    }
}

// 0 - для иностранных граждан
// 1 - для мужчин, родившихся в XIX веке
// 2 - для женщин, родившихся в XIX веке
// 3 - для мужчин, родившихся в XX веке
// 4 - для женщин, родившихся в XX веке
// 5 - для мужчин, родившихся в XXI веке
// 6 - для женщин, родившихся в XXI веке
fn n7_from_date(date: &NaiveDate, is_male: bool) -> u8 {
    let year = date.year();
    match year {
        1801..=1900 => {
            if is_male {
                1
            } else {
                2
            }
        }
        1901..=2000 => {
            if is_male {
                3
            } else {
                4
            }
        }
        2001..=2100 => {
            if is_male {
                5
            } else {
                6
            }
        }
        _ => 0,
    }
}
