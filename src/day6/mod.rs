use std::fs;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<String>,
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

fn calculate_problem(problem: &Problem) -> i64 {
    // Convert to i64
    let total = problem
        .numbers
        .clone()
        .iter()
        .map(|x| x.trim().to_string().parse().unwrap())
        .collect::<Vec<i64>>();
    // Reduce to single value based on operator
    let total = total
        .iter()
        .copied()
        .reduce(|a, b| match problem.operator.as_str() {
            "+" => a + b,
            "*" => a * b,
            _ => a,
        })
        .unwrap_or_default();

    total
}

pub(crate) async fn day6(data: Option<String>) -> (i64, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day6/data/main.txt").unwrap());

    let mut problems: Vec<Problem> = Vec::new();
    let data_lines = data.lines();
    let data_lines = data_lines.collect::<Vec<&str>>();

    // let parse_indexes: Vec<Range<usize>>;
    let mut parse_sizes: Vec<usize> = Vec::new();
    for line in data_lines.iter() {
        let parts = line.split_whitespace();
        for (part_index, part) in parts.enumerate() {
            if parse_sizes.get(part_index).is_none() {
                parse_sizes.push(part.len());
            } else if part.len() > parse_sizes[part_index] {
                parse_sizes[part_index] = part.len();
            }
        }
    }

    // Create problems from data
    for (line_index, line) in data_lines.iter().enumerate() {
        // handle last line
        if line_index == data_lines.len() - 1 {
            let parts = line.split_whitespace();
            for (part_index, part) in parts.enumerate() {
                problems[part_index].operator = part.trim().to_string();
            }
            break;
        }

        let mut cur_index = 0;
        for (size_index, size) in parse_sizes.iter().enumerate() {
            // first line - initialize problems
            if line_index == 0 {
                problems.push(Problem::new());
            }

            let cur_num = line[cur_index..(cur_index + *size)].to_string();
            problems[size_index].numbers.push(cur_num);

            cur_index += size + 1;
        }
    }

    // parse numbers for part 2
    let mut problems_part2 = problems.clone();
    for problem in problems_part2.iter_mut() {
        let mut new_numbers: Vec<String> = Vec::new();
        for num in problem.numbers.iter() {
            for (i, digit) in num.to_string().chars().enumerate() {
                if new_numbers.get(i).is_none() {
                    new_numbers.push("".to_string());
                }
                if digit == ' ' {
                    continue;
                }
                new_numbers[i] = format!("{}{}", new_numbers[i], digit);
            }
        }
        problem.numbers = new_numbers.iter().map(|x| x.to_string()).collect();
    }

    for problem in problems_part2.iter() {
        for num in problem.numbers.iter() {
            println!("{}", num);
        }
        println!();
    }

    let mut grand_total: i64 = 0;
    let mut grand_total_part2: i64 = 0;

    // Calculate problems
    for problem in problems.iter() {
        let total = calculate_problem(problem);
        grand_total += total;
    }

    for problem in problems_part2.iter() {
        let total = calculate_problem(problem);
        grand_total_part2 += total;
    }

    (grand_total, grand_total_part2)
}
