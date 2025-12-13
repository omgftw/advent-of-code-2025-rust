use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day3(data: Option<String>, digits: usize) -> i64 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day3/data/main.txt").unwrap());

    let mut total = 0;

    for line in data.lines() {
        // let mut highest_first_digit: i32 = -1;
        // let mut highest_second_digit: i32 = -1;

        let len = line.len();
        let mut highest_digits = vec![-1; digits];
        for (i, char) in line.chars().enumerate() {
            let digit = char.to_digit(10).unwrap() as i64;

            let remaining = len - i;
            let start = if remaining >= digits {
                0
            } else {
                digits - remaining
            };
            for j in start..highest_digits.len() {
                if digit > highest_digits[j] {
                    highest_digits[j] = digit;
                    for k in j + 1..highest_digits.len() {
                        highest_digits[k] = -1;
                    }
                    break;
                }
            }
        }

        let mut number = 0;
        for (i, digit) in highest_digits.iter().enumerate() {
            number += digit * 10_i64.pow((digits - i - 1) as u32);
        }

        total += number;
    }

    total
}
