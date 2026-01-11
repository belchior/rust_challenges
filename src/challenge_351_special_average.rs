/// Task 1: Special Average
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of integers.
///
/// Write a script to return the average excluding the minimum and maximum of the given array.
/// Example 1
///
/// Input: @ints = (8000, 5000, 6000, 2000, 3000, 7000)
/// Output: 5250
///
/// Min: 2000
/// Max: 8000
/// Avg: (3000+5000+6000+7000)/4 = 21000/4 = 5250
///
///
/// Example 2
///
/// Input: @ints = (100_000, 80_000, 110_000, 90_000)
/// Output: 95_000
///
/// Min: 80_000
/// Max: 110_000
/// Avg: (100_000 + 90_000)/2 = 190_000/2 = 95_000
///
///
/// Example 3
///
/// Input: @ints = (2500, 2500, 2500, 2500)
/// Output: 0
///
/// Min: 2500
/// Max: 2500
/// Avg: 0
///
///
/// Example 4
///
/// Input: @ints = (2000)
/// Output: 0
///
/// Min: 2000
/// Max: 2000
/// Avg: 0
///
///
/// Example 5
///
/// Input: @ints = (1000, 2000, 3000, 4000, 5000, 6000)
/// Output: 3500
///
/// Min: 1000
/// Max: 6000
/// Avg: (2000 + 3000 + 4000 + 5000)/4 = 14000/4 = 3500

fn special_average(list: &mut [i32]) -> f64 {
  if list.len() <= 2 {
    return 0.0;
  }

  list.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let to_exclude = &[list.first().unwrap(), list.last().unwrap()];
  let filtered = list
    .iter()
    .filter(|item| to_exclude.contains(item) == false)
    .collect::<Vec<_>>();

  let sum: i32 = filtered.iter().map(|item| *item).sum();

  if sum == 0 {
    return 0.0;
  }
  (sum / filtered.len() as i32) as f64
}

#[test]
fn test() {
  let example_1 = &mut [8000, 5000, 6000, 2000, 3000, 7000];
  let example_2 = &mut [100_000, 80_000, 110_000, 90_000];
  let example_3 = &mut [2500, 2500, 2500, 2500];
  let example_4 = &mut [2000];
  let example_5 = &mut [1000, 2000, 3000, 4000, 5000, 6000];

  assert_eq!(special_average(example_1), 5250.0);
  assert_eq!(special_average(example_2), 95_000.0);
  assert_eq!(special_average(example_3), 0.0);
  assert_eq!(special_average(example_4), 0.0);
  assert_eq!(special_average(example_5), 3500.0);
}
