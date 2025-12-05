use std::fs;

#[cfg(test)]
mod tests;

pub(crate) async fn day5(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day5/data/main.txt").unwrap());
    (0, 0)
}
