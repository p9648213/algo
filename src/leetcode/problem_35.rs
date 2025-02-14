//! https://leetcode.com/problems/search-insert-position/

// 35. Search Insert Position
// Easy
// Topics
// Companies
// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

// Example 1:

// Input: nums = [1,3,5,6], target = 5
// Output: 2
// Example 2:

// Input: nums = [1,3,5,6], target = 2
// Output: 1
// Example 3:

// Input: nums = [1,3,5,6], target = 7
// Output: 4

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search_insert_1() {
    let candidate = vec![1,3,5,6];
    let target = 5;
    let output = 2;
    let result = search_insert(candidate, target);
    assert_eq!(result, output);
  }

  #[test]
  fn test_search_insert_2() {
    let candidate = vec![1,3,5,6];
    let target = 2;
    let output = 1;
    let result = search_insert(candidate, target);
    assert_eq!(result, output);
  }

  #[test]
  fn test_search_insert_3() {
    let candidate = vec![1,3,5,6];
    let target = 7;
    let output = 4;
    let result = search_insert(candidate, target);
    assert_eq!(result, output);
  }
}