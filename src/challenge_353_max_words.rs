/// Task 1: Max Words
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given an array of sentences.
///
/// Write a script to return the maximum number of words that appear in a single sentence.
/// Example 1
///
/// Input: @sentences = ("Hello world", "This is a test", "Perl is great")
/// Output: 4
///
///
/// Example 2
///
/// Input: @sentences = ("Single")
/// Output: 1
///
///
/// Example 3
///
/// Input: @sentences = ("Short", "This sentence has seven words in total", "A B C", "Just four words here")
/// Output: 7
///
///
/// Example 4
///
/// Input: @sentences = ("One", "Two parts", "Three part phrase", "")
/// Output: 3
///
///
/// Example 5
///
/// Input: @sentences = ("The quick brown fox jumps over the lazy dog", "A", "She sells seashells by the seashore", "To be or not to be that is the question")
/// Output: 10

fn max_words(list: &[&str]) -> usize {
  list.iter().map(|s| s.split_whitespace().count()).max().unwrap()
}

#[test]
pub fn test() {
  let example_1 = &["Hello world", "This is a test", "Perl is great"];
  let example_2 = &["Single"];
  let example_3 = &[
    "Short",
    "This sentence has seven words in total",
    "A B C",
    "Just four words here",
  ];
  let example_4 = &["One", "Two parts", "Three part phrase", ""];
  let example_5 = &[
    "The quick brown fox jumps over the lazy dog",
    "A",
    "She sells seashells by the seashore",
    "To be or not to be that is the question",
  ];

  assert_eq!(max_words(example_1), 4);
  assert_eq!(max_words(example_2), 1);
  assert_eq!(max_words(example_3), 7);
  assert_eq!(max_words(example_4), 3);
  assert_eq!(max_words(example_5), 10);
}
