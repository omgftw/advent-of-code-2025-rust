use crate::day2;
use std::fs;

#[tokio::test]
async fn test_day2_test_data() {
    let test_data = fs::read_to_string("src/day2/data/test_1.txt").unwrap();
    let (part1, part2) = day2::day2(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 1227775554);
    // Part 2
    assert_eq!(part2, 4174379265);
}

#[tokio::test]
async fn test_day2() {
    let (part1, part2) = day2::day2(None).await;

    // Part 1
    assert_eq!(part1, 28146997880);
    // Part 2
    assert_eq!(part2, 40028128307);
}
