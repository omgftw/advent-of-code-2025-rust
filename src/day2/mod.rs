use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day2(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());

    let mut invalid_ids = Vec::new();

    let ranges = data.split(',').map(|x| x.split('-'));

    for mut range in ranges {
        let start = range.next().unwrap().parse::<i64>().unwrap();
        let end = range.next().unwrap().parse::<i64>().unwrap();

        for i in start..=end {
            let s = i.to_string();
            let len = s.len();
            let mid = len / 2;
            let (first_half, second_half) = s.split_at(mid);
            if first_half == second_half {
                invalid_ids.push(i);
            }
        }
    }

    let sum = invalid_ids.iter().sum::<i64>();

    (sum, 0_i64)
}
