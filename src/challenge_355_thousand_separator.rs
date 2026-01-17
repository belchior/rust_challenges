/// Task 1: Thousand Separator
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given a positive integer, $int.
///
/// Write a script to add thousand separator, , and return as string.
/// Example 1
///
/// Input: $int = 123
/// Output: "123"
///
///
/// Example 2
///
/// Input: $int = 1234
/// Output: "1,234"
///
///
/// Example 3
///
/// Input: $int = 1000000
/// Output: "1,000,000"
///
///
/// Example 4
///
/// Input: $int = 1
/// Output: "1"
///
///
/// Example 5
///
/// Input: $int = 12345
/// Output: "12,345"

fn thousand_separator(num: usize) -> String {
  let num_string = num.to_string();
  let len = num_string.len();
  let pad = match len % 3 {
    0 => "".to_string(),
    n => "*".repeat(3 - n),
  };
  format!("{pad}{num_string}")
    .chars()
    .enumerate()
    .fold(String::new(), |mut acc, (index, ch)| {
      if ch == '*' {
        return acc;
      }
      acc.push(ch);
      if (index + 1) % 3 == 0 && index + 1 < len {
        acc.push(',');
      }
      acc
    })
}

#[test]
pub fn test() {
  let example_1 = 123;
  let example_2 = 1234;
  let example_3 = 1000000;
  let example_4 = 1;
  let example_5 = 12345;
  let example_6 = 123456;

  assert_eq!(thousand_separator(example_1), "123");
  assert_eq!(thousand_separator(example_2), "1,234");
  assert_eq!(thousand_separator(example_3), "1,000,000");
  assert_eq!(thousand_separator(example_4), "1");
  assert_eq!(thousand_separator(example_5), "12,345");
  assert_eq!(thousand_separator(example_6), "123,456");
}
