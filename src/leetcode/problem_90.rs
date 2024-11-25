//! https://leetcode.com/problems/subsets-ii/description/

// Given an integer array nums that may contain duplicates, return all possible
// subsets
// (the power set).

// The solution set must not contain duplicate subsets. Return the solution in any order.

// Example 1:

// Input: nums = [1,2,2]
// Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]

// Example 2:

// Input: nums = [0]
// Output: [[],[0]]

// Time O(n * 2^n) | Space O(n)
pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    nums.sort();

    let mut result: Vec<Vec<i32>> = vec![];

    let mut subset: Vec<i32> = vec![];

    helper(&nums, 0, &mut subset, &mut result);

    result
}

fn helper(nums: &Vec<i32>, mut index: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        return result.push(subset.clone());
    }

    // include
    subset.push(nums[index]);
    helper(nums, index + 1, subset, result);
    subset.pop();

    // not include
    while index < nums.len() - 1 && nums[index] == nums[index + 1] {
        index = index + 1;
    }
    helper(nums, index + 1, subset, result);
}

pub fn sorted_subsets(mut nums: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for subset in nums.iter_mut() {
        subset.sort();
    }
    nums.sort();
    nums
}

#[cfg(test)]
mod tests {
    use super::*; // Assumes `subsets_with_dup` is in the same module or properly imported.

    fn sorted_subsets(mut subsets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for subset in &mut subsets {
            subset.sort();
        }
        subsets.sort();
        subsets
    }

    #[test]
    fn test_subsets_with_dup_case1() {
        let nums = vec![1, 3, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![3],
            vec![1, 3],
            vec![3, 3],
            vec![1, 3, 3],
        ];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_with_dup_case2() {
        let nums = vec![2, 1, 2];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![2, 2],
            vec![1, 2, 2],
        ];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_with_dup_case3() {
        let nums = vec![0, 1, 0];
        let expected = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 1],
            vec![0, 0],
            vec![0, 0, 1],
        ];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_with_dup_case4() {
        let nums = vec![2, 2, 1];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![2, 2],
            vec![1, 2],
            vec![1, 2, 2],
        ];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_with_dup_case5() {
        let nums = vec![1, 2, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![2, 2],
            vec![1, 2, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
            vec![2, 2, 3],
            vec![1, 2, 2, 3],
        ];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_with_dup_case6() {
        let nums = vec![2, 2, 2, 2];
        let expected = vec![vec![], vec![2], vec![2, 2], vec![2, 2, 2], vec![2, 2, 2, 2]];
        let result = subsets_with_dup(nums.clone());
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }
}
