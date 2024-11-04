//! https://leetcode.com/problems/squares-of-a-sorted-array/description/
//! Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

//? Example 1:

// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
// Explanation: After squaring, the array becomes [16,1,0,9,100].
// After sorting, it becomes [0,1,9,16,100].

//? Example 2:

// Input: nums = [-7,-3,2,3,11]
// Output: [4,9,9,49,121]

//* Time: O(nlogn) | Space: O(n)
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut nums: Vec<i32> = nums.iter().map(|x| x.pow(2)).collect();
    nums.sort();
    nums
}

//* Time: O(n) | Space: O(n)
pub fn sorted_squares_2(nums: Vec<i32>) -> Vec<i32> {
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut index = end;
    let mut result: Vec<i32> = vec![0; nums.len()];

    for _ in nums.iter() {
        if nums[start].pow(2) > nums[end].pow(2) {
            result[index] = nums[start].pow(2);
            start = start + 1;
        } else {
            result[index] = nums[end].pow(2);
            end = end - 1;
        }
        if index > 0 {
            index = index - 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_977() {
        let result = sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);

        let result = sorted_squares(vec![-7, -3, 2, 3, 11]);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);

        let result = sorted_squares_2(vec![-4, -1, 0, 3, 10]);
        assert_eq!(result, vec![0, 1, 9, 16, 100]);

        let result = sorted_squares_2(vec![-7, -3, 2, 3, 11]);
        assert_eq!(result, vec![4, 9, 9, 49, 121]);
    }
}
