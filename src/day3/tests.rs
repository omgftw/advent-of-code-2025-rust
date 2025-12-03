use crate::day3;
use std::fs;

#[tokio::test]
async fn test_day3_test_data() {
    let test_data = fs::read_to_string("src/day3/data/test_1.txt").unwrap();
    let (part1, part2) = day3::day3(Some(test_data)).await;

    // Part 1
    assert_eq!(part1, 357);
    // Part 2
    // assert_eq!(part2, 1);
}

// #[tokio::test]
// async fn test_day3() {
//     let (part1, part2) = day3::day3(None).await;

//     // Part 1
//     assert_eq!(part1, 1);
//     // Part 2
//     assert_eq!(part2, 1);
// }
