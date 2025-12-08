use crate::day5;
use std::fs;

#[tokio::test]
async fn test_day5_test_data() {
    let test_data = fs::read_to_string("src/day5/data/test_1.txt").unwrap();
    let (part1, part2) = day5::day5(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 3);
    // Part 2
    assert_eq!(part2, 14);
}

#[tokio::test]
async fn test_day5() {
    let (part1, part2) = day5::day5(None).await;

    // Part 1
    assert_eq!(part1, 773);
    // Part 2
    assert_eq!(part2, 332067203034711);
}
