use std::fs;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: String,
}

impl Problem {
    fn new() -> Problem {
        Problem {
            numbers: vec![],
            operator: "".to_string(),
        }
    }
}

pub(crate) async fn day6(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day6/data/main.txt").unwrap());

    let mut problems: Vec<Problem> = Vec::new();
    let data_lines = data.lines();
    let data_lines = data_lines.collect::<Vec<&str>>();

    // Create problems from data
    for (line_index, line) in data_lines.iter().enumerate() {
        let parts = line.split_whitespace();
        for (part_index, part) in parts.enumerate() {
            // last line
            if line_index == data_lines.len() - 1 {
                problems[part_index].operator = part.to_string();
            } else {
                // first line - initialize problems
                if line_index == 0 {
                    problems.push(Problem::new());
                }
                problems[part_index]
                    .numbers
                    .push(part.to_string().parse().unwrap())
            }
        }
    }

    let mut grand_total: i64 = 0;
    // Calculate problems

    for problem in problems.iter() {
        // let mut total: &i64 = 0;
        let total = problem.numbers.iter().reduce(|a, b| match problem.operator.as_str() {
            "+" => a + b,
            "*" => a * b,
            _ => {},
        });
        // for num in problem.numbers.iter() {
        //     match problem.operator.as_str() {
        //         "+" => total += num,
        //         "*" => total *= num,
        //         _ => {},
        //     }
        // }
        println!("{}", total);

        grand_total += total;
    }

    (grand_total, 0)
}
