use itertools::Itertools;
use std::ops::Sub;

/// Task 2: Arithmetic Progression
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of numbers.
///
/// Write a script to return true if the given array can be re-arranged to form an arithmetic progression, otherwise return false.
///
/// > A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.
///
/// Example 1
///
/// Input: @num = (1, 3, 5, 7, 9)
/// Output: true
///
/// Already AP with common difference 2.
///
///
/// Example 2
///
/// Input: @num = (9, 1, 7, 5, 3)
/// Output: true
///
/// The given array re-arranged like (1, 3, 5, 7, 9) with common difference 2.
///
///
/// Example 3
///
/// Input: @num = (1, 2, 4, 8, 16)
/// Output: false
///
/// This is geometric progression and not arithmetic progression.
///
///
/// Example 4
///
/// Input: @num = (5, -1, 3, 1, -3)
/// Output: true
///
/// The given array re-arranged like (-3, -1, 1, 3, 5) with common difference 2.
///
///
/// Example 5
///
/// Input: @num = (1.5, 3, 0, 4.5, 6)
/// Output: true
///
/// The given array re-arranged like (0, 1.5, 3, 4.5, 6) with common difference 1.5.

fn can_be_arithmetic_progression<T: Copy + PartialOrd + Sub<Output = T>>(list: &[T]) -> bool {
  if list.len() <= 1 {
    return false;
  }
  list
    .into_iter()
    .sorted_by(|a, b| a.partial_cmp(b).unwrap())
    .tuple_windows()
    .map(|(a, b)| *b - *a)
    .all_equal()
}

#[test]
pub fn test() {
  let example_1 = &[1, 3, 5, 7, 9];
  let example_2 = &[9, 1, 7, 5, 3];
  let example_3 = &[1, 2, 4, 8, 16];
  let example_4 = &[5, -1, 3, 1, -3];
  let example_5 = &[1.5, 3.0, 0.0, 4.5, 6.0];
  let example_6 = &[1];
  let example_7: &[i32] = &[];

  assert_eq!(can_be_arithmetic_progression(example_1), true);
  assert_eq!(can_be_arithmetic_progression(example_2), true);
  assert_eq!(can_be_arithmetic_progression(example_3), false);
  assert_eq!(can_be_arithmetic_progression(example_4), true);
  assert_eq!(can_be_arithmetic_progression(example_5), true);
  assert_eq!(can_be_arithmetic_progression(example_6), false);
  assert_eq!(can_be_arithmetic_progression(example_7), false);
}
