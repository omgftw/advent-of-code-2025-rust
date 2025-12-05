use std::fs;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector2 {
    x: usize,
    y: usize,
}

// fn is_safe(pos: &Vector2, max_bounds: &Vector2) -> bool {
//     if pos.x > 0 && pos.x < max_bounds.x - 1 && pos.y > 0 && pos.y < max_bounds.y - 1 {
//         return true;
//     }
//     false
// }

// fn get_adjacent_count(pos: &Vector2, data: &Vec<String>, max_bounds: &Vector2) -> i64 {
//     let mut count = 0;

//     for x in pos.x-1..pos.x+1 {
//         for y in pos.y-1..pos.y+1 {
//             let cur = Vector2 { x, y };
//             if cur == *pos {
//                 continue;
//             }
//             if is_safe(&cur, &max_bounds) {
//                 if data[cur.y][cur.x] == "@" {

//                 }
//                 count += 1;
//             }
//         }
//     }

//     count
// }

type Row = Vec<char>;

#[derive(Clone, Debug)]
struct Grid {
    rows: Vec<Row>,
    max_bounds: Option<Vector2>,
}

impl Grid {
    fn get_pos(&mut self, pos: Vector2) -> &mut char {
        &mut self.rows[pos.y][pos.x]
    }

    fn get_max_bounds(&mut self) -> Vector2 {
        let bounds = self.max_bounds.clone().unwrap_or(Vector2 {
            x: self.rows[0].len() - 1,
            y: self.rows.len() - 1,
        });

        if self.max_bounds.is_none() {
            self.max_bounds = Some(bounds);
        }

        bounds
    }

    fn get_adjacent_count(&mut self, pos: &Vector2) -> i64 {
        let mut count = 0;
        let max_bounds = self.get_max_bounds();

        for y in pos.y.saturating_sub(1)..=(pos.y + 1).min(max_bounds.y) {
            for x in pos.x.saturating_sub(1)..=(pos.x + 1).min(max_bounds.x) {
                let cur = Vector2 { x, y };
                if cur == *pos {
                    continue;
                }
                if *self.get_pos(cur) == '@' {
                    count += 1;
                }
            }
        }

        count
    }
}

pub(crate) async fn day4(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day4/data/main.txt").unwrap());

    let rows = data.lines();
    let rows: Vec<Vec<char>> = rows.collect::<Vec<&str>>().iter().map(|x| x.chars().collect()).collect();
    let mut grid = Grid {
        rows,
        max_bounds: None,
    };
    let max_bounds = grid.get_max_bounds();

    let adjacent_less_than = 4;
    let mut total = 0;

    let mut new_map = grid.clone();

    println!("Ranges are from 0 to {}", max_bounds.y);
    println!();
    println!("Ranges are from 0 to {}", max_bounds.x);
    println!();

    for y in 0..=max_bounds.y {
        for x in 0..=max_bounds.x {
            let pos = Vector2 {x, y};
            if *grid.get_pos(pos) != '@' {
                continue;
            }

            let count = grid.get_adjacent_count(&pos);
            if count < adjacent_less_than {
                total += 1;
                *new_map.get_pos(pos) = 'x';
            }
        }
    }

    // for row in new_map.rows {
    //     println!("{}", row.iter().collect::<String>());
    // }

    (total, 0)
}
