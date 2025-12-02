use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day2(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day2/data/main.txt").unwrap());

    let mut invalid_ids = Vec::new();
    let mut invalid_ids_2 = Vec::new();

    let ranges = data.split(',').map(|x| x.split('-'));

    for mut range in ranges {
        let start = range.next().unwrap().parse::<i64>().unwrap();
        let end = range.next().unwrap().parse::<i64>().unwrap();

        for i in start..=end {
            let s = i.to_string();
            if s.chars().nth(0).unwrap() == '0' {
                continue;
            }
            let len = s.len();
            let mid = len / 2;
            let (first_half, second_half) = s.split_at(mid);
            if first_half == second_half {
                invalid_ids.push(i);
            }

            for j in 0..s.len() - 1 {
                // If the length is not divisible by the current index + 1, then it cannot be a purely repeated sequence
                if len % (j + 1) != 0 {
                    continue;
                }
                let cur = s[0..=j].to_string();
                let repeats = cur.repeat(len / (j + 1));
                if repeats == s {
                    invalid_ids_2.push(i);
                    break;
                }
            }
        }
    }



    // println!("invalid_ids_2: {:?}", invalid_ids_2);

    let sum = invalid_ids.iter().sum::<i64>();
    let sum_2 = invalid_ids_2.iter().sum::<i64>();
    (sum, sum_2)
}
