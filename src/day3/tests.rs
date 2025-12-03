use crate::day3;
use std::fs;

#[tokio::test]
async fn test_day3_test_data() {
    let test_data = fs::read_to_string("src/day3/data/test_1.txt").unwrap();
    let part1 = day3::day3(Some(test_data.clone()), 2).await;
    let part2 = day3::day3(Some(test_data), 12).await;

    // Part 1
    assert_eq!(part1, 357);
    // Part 2
    assert_eq!(part2, 3121910778619);
}

#[tokio::test]
async fn test_day3() {
    let part1 = day3::day3(None, 2).await;
    let part2 = day3::day3(None, 12).await;

    // Part 1
    assert_eq!(part1, 17095);
    // Part 2
    assert_eq!(part2, 168794698570517);
}
