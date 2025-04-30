// https://leetcode.com/problems/two-sum/description/

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

//!-----------------------------------------------------------

// Time: O(n^2) | Space: O(1)
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     if nums.len() <= 1 {
//         return vec![];
//     }

//     for index1 in 0..(nums.len() - 1) {
//         for index2 in index1 + 1..nums.len() {
//             if nums[index1] + nums[index2] == target {
//                 return vec![index1 as i32, index2 as i32];
//             }
//         }
//     }

//     return vec![];
// }

//!-----------------------------------------------------------

use std::collections::HashMap;

// Time: O(n) | Space: O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h_table: HashMap<i32, i32> = HashMap::new();

    if nums.len() <= 1 {
        return vec![];
    }

    for (index, num) in nums.iter().enumerate() {
        let needed_num = target - num;
        if h_table.contains_key(&needed_num) {
            return vec![index as i32, *h_table.get(&needed_num).unwrap()];
        } else {
            h_table.insert(*num, index as i32);
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to check if two vectors contain the same elements
    fn arrays_contain_same_elements(a: Vec<i32>, b: Vec<i32>) -> bool {
        let mut a_sorted = a.to_vec();
        let mut b_sorted = b.to_vec();
        a_sorted.sort_unstable();
        b_sorted.sort_unstable();
        a_sorted == b_sorted
    }

    #[test]
    fn test_empty_array() {
        let result = two_sum(vec![], 10);
        assert_eq!(
            result,
            vec![],
            "Expected an empty array for an empty input array"
        );
    }

    #[test]
    fn test_valid_solution() {
        let result1 = two_sum(vec![2, 7, 11, 15], 9);
        println!("{:?}", result1);
        assert!(
            arrays_contain_same_elements(result1, vec![0, 1]),
            "Expected indices [0, 1] for array [2, 7, 11, 15] and target 9"
        );

        let result2 = two_sum(vec![3, 2, 4], 6);
        println!("{:?}", result2);
        assert!(
            arrays_contain_same_elements(result2, vec![1, 2]),
            "Expected indices [1, 2] for array [3, 2, 4] and target 6"
        );

        let result3 = two_sum(vec![0, 4, 3, 0], 0);
        println!("{:?}", result3);
        assert!(
            arrays_contain_same_elements(result3, vec![0, 3]),
            "Expected indices [0, 3] for array [0, 4, 3, 0] and target 0"
        );

        let result4 = two_sum(vec![2, 5, 5, 11], 10);
        println!("{:?}", result4);
        assert!(
            arrays_contain_same_elements(result4, vec![1, 2]),
            "Expected indices [1, 2] for array [2, 5, 5, 11] and target 0"
        );
    }

    #[test]
    fn test_no_valid_solution() {
        let result = two_sum(vec![1, 2, 3, 4, 5], 10);
        assert_eq!(
            result,
            vec![],
            "Expected an empty array for no valid solution"
        );
    }
}
