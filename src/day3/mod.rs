use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day3(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day3/data/main.txt").unwrap());

    let mut total = 0;

    for line in data.lines() {
        let mut highest_first_digit: i32 = -1;
        let mut highest_second_digit: i32 = -1;

        let len = line.len();
        for (i, char) in line.chars().enumerate() {
            let digit = char.to_digit(10).unwrap() as i32;
            if digit > highest_second_digit {
                highest_second_digit = digit;
            }
            // Don't check the last character for the highest first digit
            if digit > highest_first_digit && i != len - 1 {
                highest_first_digit = digit;
                highest_second_digit = -1;
            }
        }

        total += highest_first_digit * 10 + highest_second_digit;
        println!("{}", highest_first_digit * 10 + highest_second_digit);
    }

    (total, 0)
}
