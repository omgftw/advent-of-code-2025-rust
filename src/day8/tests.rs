use crate::day8;
use std::fs;

#[tokio::test]
async fn test_day8_test_data() {
    let test_data = fs::read_to_string("src/day8/data/test_1.txt").unwrap();
    let (part1, part2) = day8::day8(Some(test_data), 10).await;

    // Part 1
    assert_eq!(part1, 40);
    // Part 2
    // assert_eq!(part2, 1);
}

#[tokio::test]
async fn test_day8() {
    let (part1, part2) = day8::day8(None, 1000).await;

    // Part 1
    assert_eq!(part1, 1);
    // Part 2
    assert_eq!(part2, 1);
}
