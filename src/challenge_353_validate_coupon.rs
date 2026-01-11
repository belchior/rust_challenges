use itertools::izip;
use regex::Regex;

/// Task 2: Validate Coupon
/// Submitted by: Mohammad Sajid Anwar
///
/// You are given three arrays, @codes, @names and @status.
///
/// Write a script to validate codes in the given array.
///
/// A code is valid when the following conditions are true:
/// - codes[i] is non-empty and consists only of alphanumeric characters (a-z, A-Z, 0-9) and underscores (_).
/// - names[i] is one of the following four categories: "electronics", "grocery", "pharmacy", "restaurant".
/// - status[i] is true.
///
/// Return an array of booleans indicating validity: output[i] is true if and only if codes[i], names[i] and status[i] are all valid.
/// Example 1
///
/// Input: @codes  = ("A123",        "B_456",      "C789",        "D@1",      "E123")
///        @names  = ("electronics", "restaurant", "electronics", "pharmacy", "grocery")
///        @status = ("true",        "false",      "true",        "true",     "true")
/// Output: (true, false, true, false, true)
///
///
/// Example 2
///
/// Input: @codes  = ("Z_9", "AB_12", "G01", "X99", "test")
///        @names  = ("pharmacy", "electronics", "grocery", "electronics", "unknown")
///        @status = ("true", "true", "false", "true", "true")
/// Output: (true, true, false, true, false)
///
///
/// Example 3
///
/// Input: @codes  = ("_123", "123", "", "Coupon_A", "Alpha")
///        @names  = ("restaurant", "electronics", "electronics", "pharmacy", "grocery")
///        @status = ("true", "true", "false", "true", "true")
/// Output: (true, true, false, true, true)
///
///
/// Example 4
///
/// Input: @codes  = ("ITEM_1", "ITEM_2", "ITEM_3", "ITEM_4")
///        @names  = ("electronics", "electronics", "grocery", "grocery")
///        @status = ("true", "true", "true", "true")
/// Output: (true, true, true, true)
///
///
/// Example 5
///
/// Input: @codes  = ("CAFE_X", "ELEC_100", "FOOD_1", "DRUG_A", "ELEC_99")
///        @names  = ("restaurant", "electronics", "grocery", "pharmacy", "electronics")
///        @status = ("true", "true", "true", "true", "false")
/// Output: (true, true, true, true, false)

fn validate_coupon<'a>(input: Input<'a>, re: &Regex, names: &[&str]) -> Vec<bool> {
  izip!(input.codes, input.names, input.status)
    .map(|(a, b, c)| re.captures(a).is_some() && names.contains(b) && *c == "true")
    .collect::<Vec<_>>()
}

struct Input<'a> {
  codes: &'a [&'a str],
  names: &'a [&'a str],
  status: &'a [&'a str],
}

#[test]
pub fn test() {
  let example_1 = Input {
    codes: &["A123", "B_456", "C789", "D@1", "E123"],
    names: &["electronics", "restaurant", "electronics", "pharmacy", "grocery"],
    status: &["true", "false", "true", "true", "true"],
  };
  let example_2 = Input {
    codes: &["Z_9", "AB_12", "G01", "X99", "test"],
    names: &["pharmacy", "electronics", "grocery", "electronics", "unknown"],
    status: &["true", "true", "false", "true", "true"],
  };
  let example_3 = Input {
    codes: &["_123", "123", "", "Coupon_A", "Alpha"],
    names: &["restaurant", "electronics", "electronics", "pharmacy", "grocery"],
    status: &["true", "true", "false", "true", "true"],
  };
  let example_4 = Input {
    codes: &["ITEM_1", "ITEM_2", "ITEM_3", "ITEM_4"],
    names: &["electronics", "electronics", "grocery", "grocery"],
    status: &["true", "true", "true", "true"],
  };
  let example_5 = Input {
    codes: &["CAFE_X", "ELEC_100", "FOOD_1", "DRUG_A", "ELEC_99"],
    names: &["restaurant", "electronics", "grocery", "pharmacy", "electronics"],
    status: &["true", "true", "true", "true", "false"],
  };

  let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
  let names = &["electronics", "grocery", "pharmacy", "restaurant"];

  assert_eq!(
    validate_coupon(example_1, &re, names),
    &[true, false, true, false, true]
  );
  assert_eq!(
    validate_coupon(example_2, &re, names),
    &[true, true, false, true, false]
  );
  assert_eq!(validate_coupon(example_3, &re, names), &[true, true, false, true, true]);
  assert_eq!(validate_coupon(example_4, &re, names), &[true, true, true, true]);
  assert_eq!(validate_coupon(example_5, &re, names), &[true, true, true, true, false]);
}
