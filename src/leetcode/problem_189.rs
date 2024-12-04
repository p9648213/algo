// https://leetcode.com/problems/rotate-array/description/

// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

// Example 1:

// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]

// Example 2:

// Input: nums = [-1,-100,3,99], k = 2
// Output: [3,99,-1,-100]
// Explanation:
// rotate 1 steps to the right: [99,-1,-100,3]
// rotate 2 steps to the right: [3,99,-1,-100]

// Time: O(n)  | Space: O(n)
// pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
//     let mut k = k;
//     if k == nums.len() as i32 || k == 0 || nums.len() == 0 || nums.len() == 1 {
//         return;
//     }

//     if k > nums.len() as i32 {
//         k = k % nums.len() as i32;
//     }

//     let result = [
//         &nums[(nums.len() - k as usize)..],
//         &nums[..(nums.len() - k as usize)],
//     ]
//     .concat();

//     nums.clear();
//     nums.extend(result);
// }

// Time: O(n)  | Space: O(1)
pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
    let mut k = k;
    if k == nums.len() as i32 || k == 0 || nums.len() == 0 || nums.len() == 1 {
        return;
    }

    if k > nums.len() as i32 {
        k = k % nums.len() as i32;
    }

    nums.reverse();

    let n = nums.len();

    reverse_range(nums, 0, (k - 1) as usize);
    reverse_range(nums, k as usize, n - 1);
}

fn reverse_range(arr: &mut [i32], start: usize, end: usize) {
    let mut left = start;
    let mut right = end;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the rotate_array function from the main module

    #[test]
    fn test_empty_array() {
        let mut array = vec![];
        rotate_array(&mut array, 3);
        assert_eq!(array, vec![]);
    }

    #[test]
    fn test_rotate_array() {
        let mut array = vec![1, 2, 3, 4, 5];
        rotate_array(&mut array, 2);
        assert_eq!(array, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_rotate_with_k_greater_than_length() {
        let mut array = vec![1, 2, 3, 4, 5];
        rotate_array(&mut array, 8);
        assert_eq!(array, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_rotate_with_k_equal_to_length() {
        let mut array = vec![1, 2, 3, 4, 5];
        rotate_array(&mut array, 5);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_rotate_with_k_zero() {
        let mut array = vec![1, 2, 3, 4, 5];
        rotate_array(&mut array, 0);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_rotate_with_k_1() {
        let mut array = vec![-1, -100, 3, 99];
        rotate_array(&mut array, 2);
        assert_eq!(array, vec![3, 99, -1, -100]);
    }

    // #[test]
    // fn test_rotate_with_negative_k() {
    //     let mut array = vec![1, 2, 3, 4, 5];
    //     rotate_array(&mut array, -3);
    //     assert_eq!(array, vec![3, 4, 5, 1, 2]);
    // }
}
