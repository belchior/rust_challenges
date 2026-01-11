use itertools::Itertools;

/// Task 1: Min Abs Diff
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of distinct integers.
///
/// Write a script to find all pairs of elements with the minimum absolute difference.
///
/// Rules (a,b):
/// 1: a, b are from the given array.
/// 2: a < b
/// 3: b - a = min abs diff any two elements in the given array
///
/// Example 1
///
/// Input: @ints= (4, 2, 1, 3)
/// Output: [1, 2], [2, 3], [3, 4]
///
///
/// Example 2
///
/// Input: @ints = (10, 100, 20, 30)
/// Output: [10, 20], [20, 30]
///
///
/// Example 3
///
/// Input: @ints = (-5, -2, 0, 3)
/// Output: [-2, 0]
///
///
/// Example 4
///
/// Input: @ints = (8, 1, 15, 3)
/// Output: [1, 3]
///
///
/// Example 5
///
/// Input: @ints = (12, 5, 9, 1, 15)
/// Output: [9, 12], [12, 15]

fn min_abs_diff(list: &[i32]) -> Vec<(i32, i32)> {
  if list.len() <= 1 {
    return vec![];
  }
  list
    .into_iter()
    .sorted()
    .tuple_windows()
    .map(|(a, b)| [*a, *b, (*a - *b).abs()])
    .fold(Vec::<[i32; 3]>::new(), |mut acc: Vec<_>, win| {
      if acc.is_empty() || acc[0][2] == win[2] {
        acc.push(win);
      } else if acc[0][2] > win[2] {
        acc = vec![win]
      }
      acc
    })
    .iter()
    .map(|item| (item[0], item[1]))
    .collect()
}

#[test]
pub fn test() {
  let example_1 = &[4, 2, 1, 3];
  let example_2 = &[10, 100, 20, 30];
  let example_3 = &[-5, -2, 0, 3];
  let example_4 = &[8, 1, 15, 3];
  let example_5 = &[12, 5, 9, 1, 15];
  let example_6 = &[125];

  assert_eq!(min_abs_diff(example_1), vec![(1, 2), (2, 3), (3, 4)]);
  assert_eq!(min_abs_diff(example_2), vec![(10, 20), (20, 30)]);
  assert_eq!(min_abs_diff(example_3), vec![(-2, 0)]);
  assert_eq!(min_abs_diff(example_4), vec![(1, 3)]);
  assert_eq!(min_abs_diff(example_5), vec![(9, 12), (12, 15)]);
  assert_eq!(min_abs_diff(example_6), vec![]);
}
