use std::fs;

use log::debug;

#[cfg(test)]
mod tests;

struct Rotation {
    direction: String,
    amount: i32,
}

// Poor implementation
pub(crate) async fn day1(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day1/data/main.txt").unwrap());

    let mut instructions: Vec<Rotation> = Vec::new();

    for line in data.lines() {
        let parts = line.split_at(1);
        let rotation = Rotation {
            direction: parts.0.to_string(),
            amount: parts.1.parse::<i32>().unwrap(),
        };

        instructions.push(rotation);
    }

    let mut value = 50;
    let mut times_end_on_zero = 0;
    let mut times_zero_reached = 0;

    for instruction in instructions.iter_mut() {
        let mult = match instruction.direction.as_str() {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };

        let mut prev_val = -1;
        for _ in 0..instruction.amount {
            if prev_val == 0 {
                times_zero_reached += 1;
            }
            value += mult;
            if value > 99 {
                value = 0;
            } else if value < 0 {
                value = 99;
            }
            prev_val = value;
        }

        if value == 0 {
            times_end_on_zero += 1;
        }

        debug!(
            "{} - {} - {} - {} - {}",
            instruction.direction, instruction.amount, value, times_end_on_zero, times_zero_reached
        );
    }

    (times_end_on_zero, times_end_on_zero + times_zero_reached)
}
