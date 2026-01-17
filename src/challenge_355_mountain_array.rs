/// Task 2: Mountain Array
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of integers, @ints.
///
/// Write a script to return true if the given array is a valid mountain array.
///
/// An array is mountain if and only if:
/// 1) arr.length >= 3
/// and
/// 2) There exists some i with 0 < i < arr.length - 1 such that:
/// arr[0] < arr[1]     < ... < arr[i - 1] < arr[i]
/// arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
///
/// Example 1
///
/// Input: @ints = (1, 2, 3, 4, 5)
/// Output: false
///
///
/// Example 2
///
/// Input: @ints = (0, 2, 4, 6, 4, 2, 0)
/// Output: true
///
///
/// Example 3
///
/// Input: @ints = (5, 4, 3, 2, 1)
/// Output: false
///
///
/// Example 4
///
/// Input: @ints = (1, 3, 5, 5, 4, 2)
/// Output: false
///
///
/// Example 5
///
/// Input: @ints = (1, 3, 2)
/// Output: true

fn mountain_array(list: &[i32]) -> bool {
  let len = list.len();

  if len <= 2 || len % 2 == 0 {
    return false;
  }

  let pos = ((len / 2) as f64).trunc() as usize;
  let top = list[pos];

  list.iter().enumerate().fold(true, |acc, (index, value)| {
    if acc == false || index != pos && *value >= top {
      return false;
    }
    acc
  })
}

#[test]
pub fn test() {
  let example_1 = &[1, 2, 3, 4, 5];
  let example_2 = &[0, 2, 4, 6, 4, 2, 0];
  let example_3 = &[5, 4, 3, 2, 1];
  let example_4 = &[1, 3, 5, 5, 4, 2];
  let example_5 = &[1, 3, 2];
  let example_6 = &[1];

  assert_eq!(mountain_array(example_1), false);
  assert_eq!(mountain_array(example_2), true);
  assert_eq!(mountain_array(example_3), false);
  assert_eq!(mountain_array(example_4), false);
  assert_eq!(mountain_array(example_5), true);
  assert_eq!(mountain_array(example_6), false);
}
