// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
// Example 2:

// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.

// Constraints:

// 1 <= haystack.length, needle.length <= 104
// haystack and needle consist of only lowercase English characters.

pub fn str_str(haystack: String, needle: String) -> i32 {
    0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_str_str_1() {
    let result = str_str("sadbutsad".to_owned(), "sad".to_owned());
    assert_eq!(result, 0);
  }

  #[test]
  fn test_str_str_2() {
    let result = str_str("leetcode".to_owned(), "leeto".to_owned());
    assert_eq!(result, -1);
  }
}