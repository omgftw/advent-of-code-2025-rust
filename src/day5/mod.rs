use std::fs;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn in_range(&self, num: i64) -> bool {
        if num >= self.start && num <= self.end {
            return true;
        }
        false
    }
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mut ranges = ranges.clone();
    ranges.sort_by_key(|x| x.start);

    let mut merged = vec![ranges[0]];
    for range in ranges.iter().skip(1) {
        let previous = merged.last_mut().unwrap();
        if range.start <= previous.end {
            previous.end = previous.end.max(range.end)
        } else {
            merged.push(*range);
        }
    }

    merged
}

pub(crate) async fn day5(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day5/data/main.txt").unwrap());

    let mut split_data = data.split("\n\n");
    let fresh_ingredients = split_data.next().unwrap();
    let fresh_ingredients = fresh_ingredients.split_whitespace();
    let available_ingredients = split_data.next().unwrap();
    let available_ingredients = available_ingredients.split_whitespace();

    let mut fresh_ingredient_ranges: Vec<Range> = Vec::new();

    for ingredient in fresh_ingredients {
        let mut range_parts = ingredient.split('-');
        let range = Range {
            start: range_parts.next().unwrap().to_string().parse::<i64>().unwrap(),
            end: range_parts.next().unwrap().to_string().parse::<i64>().unwrap(),
        };
        fresh_ingredient_ranges.push(range);
    }

    println!();

    let mut fresh_ingredient_count = 0;

    for ingredient in available_ingredients {
        let ingredient_id = ingredient.to_string().parse::<i64>().unwrap();
        for range in fresh_ingredient_ranges.iter() {
            if range.in_range(ingredient_id) {
                fresh_ingredient_count += 1;
                break;
            }
        }
        // println!("{}", x);
    }

    // combine ranges
    let merged_ranges = merge_ranges(&fresh_ingredient_ranges);

    let mut all_fresh_ingredient_count = 0;
    for range in merged_ranges.iter() {
        all_fresh_ingredient_count += range.end - range.start + 1
    }

    (fresh_ingredient_count, all_fresh_ingredient_count)
}
