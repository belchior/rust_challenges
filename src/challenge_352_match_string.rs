use itertools::Itertools;

/// Task 1: Match String
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of strings.
///
/// Write a script to return all strings that are a substring of another word in the given array in the order they occur.
/// Example 1
///
/// Input: @words = ("cat", "cats", "dog", "dogcat", "dogcat", "rat", "ratcatdogcat")
/// Output: ("cat", "dog", "dogcat", "rat")
///
///
/// Example 2
///
/// Input: @words = ("hello", "hell", "world", "wor", "ellow", "elloworld")
/// Output: ("hell", "world", "wor", "ellow")
///
///
/// Example 3
///
/// Input: @words = ("a", "aa", "aaa", "aaaa")
/// Output: ("a", "aa", "aaa")
///
///
/// Example 4
///
/// Input: @words = ("flower", "flow", "flight", "fl", "fli", "ig", "ght")
/// Output: ("flow", "fl", "fli", "ig", "ght")
///
///
/// Example 5
///
/// Input: @words = ("car", "carpet", "carpenter", "pet", "enter", "pen", "pent")
/// Output: ("car", "pet", "enter", "pen", "pent")

fn match_string<'a>(list: &'a [&'a str]) -> Vec<&'a str> {
  list
    .iter()
    .permutations(2)
    .filter(|item| (*item[1]).contains(*item[0]))
    .map(|item| *item[0])
    .dedup()
    .collect()
}

#[test]
pub fn test() {
  let example_1 = &mut ["cat", "cats", "dog", "dogcat", "dogcat", "rat", "ratcatdogcat"];
  let example_2 = &mut ["hello", "hell", "world", "wor", "ellow", "elloworld"];
  let example_3 = &mut ["a", "aa", "aaa", "aaaa"];
  let example_4 = &mut ["flower", "flow", "flight", "fl", "fli", "ig", "ght"];
  let example_5 = &mut ["car", "carpet", "carpenter", "pet", "enter", "pen", "pent"];

  assert_eq!(match_string(example_1), &["cat", "dog", "dogcat", "rat"]);
  assert_eq!(match_string(example_2), &["hell", "world", "wor", "ellow"]);
  assert_eq!(match_string(example_3), &["a", "aa", "aaa"]);
  assert_eq!(match_string(example_4), &["flow", "fl", "fli", "ig", "ght"]);
  assert_eq!(match_string(example_5), &["car", "pet", "enter", "pen", "pent"]);
}
