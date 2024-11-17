// https://leetcode.com/problems/monotonic-array/description/

// An array is monotonic if it is either monotone increasing or monotone decreasing.

// An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j]. An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].

// Given an integer array nums, return true if the given array is monotonic, or false otherwise.

// Example 1:

// Input: nums = [1,2,2,3]
// Output: true

// Example 2:

// Input: nums = [6,5,4,4]
// Output: true

// Example 3:

// Input: nums = [1,3,2]
// Output: false

// Time: O(n) | Space: O(1)
pub fn is_monotonic(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let is_increasing = nums[0] < nums[nums.len() - 1];
    let is_decreasing = nums[0] > nums[nums.len() - 1];
    let equal = nums[0] == nums[nums.len() - 1];

    for (index, num) in nums.iter().enumerate() {
        if index <= nums.len() - 2 {
            if is_increasing && *num > nums[index + 1] {
                return false;
            }

            if is_decreasing && *num < nums[index + 1] {
                return false;
            }

            if equal && *num != nums[index + 1] {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_896() {
        let result = is_monotonic([1, 2, 2, 3].to_vec());
        assert_eq!(result, true);

        let result = is_monotonic([6, 5, 4, 4].to_vec());
        assert_eq!(result, true);

        let result = is_monotonic([1, 3, 2].to_vec());
        assert_eq!(result, false);
    }
}
