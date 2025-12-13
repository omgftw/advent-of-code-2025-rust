use std::{collections::HashSet, fs};

#[cfg(test)]
mod tests;

fn find_start(first_line: &str) -> usize {
    for (i, char) in first_line.chars().enumerate() {
        if char == 'S' {
            return i;
        }
    }
    panic!("Could not find start position");
}

pub(crate) async fn day7(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day7/data/main.txt").unwrap());

    let lines: Vec<&str> = data.lines().collect();
    let start_index = find_start(lines[0]);
    let mut beams_indexes: HashSet<usize> = HashSet::new();
    beams_indexes.insert(start_index);

    let mut split_count = 0;

    for line in lines.iter() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for beam_index in beams_indexes.iter() {
            if line.chars().nth(*beam_index) == Some('^') {
                new_beams.insert(*beam_index - 1);
                new_beams.insert(*beam_index + 1);
                split_count += 1;
            } else {
                new_beams.insert(*beam_index);
            }
        }
        beams_indexes = new_beams;
    }

    (split_count, 0)
}
