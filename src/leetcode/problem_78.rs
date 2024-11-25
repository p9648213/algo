//! https://leetcode.com/problems/subsets/description/

// Given an integer array nums of unique elements, return all possible
// subsets
// (the power set).

// The solution set must not contain duplicate subsets. Return the solution in any order.

// Example 1:

// Input: nums = [1,2,3]
// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

// Example 2:

// Input: nums = [0]
// Output: [[],[0]]

// Time O(n * 2^n) | Space O(n)
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![]];

    for num in nums.iter() {
        let mut new_subsets = vec![];

        if result.len() > 0 {
            for subset in result.iter() {
                let mut new_subset = subset.clone();
                new_subset.push(*num);
                new_subsets.push(new_subset);
            }
        }

        result.append(&mut new_subsets);
    }

    result
}

// Time O(n * 2^n) | Space O(n)
pub fn subsets_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    let mut subset: Vec<i32> = vec![];

    helper(&nums, 0, &mut subset, &mut result);

    result
}

fn helper(nums: &Vec<i32>, index: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if index == nums.len() {
        return result.push(subset.clone());
    }

    // add
    subset.push(nums[index]);
    helper(nums, index + 1, subset, result);
    subset.pop();

    // don't add
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
    use super::*;

    #[test]
    fn test_subsets_1_2() {
        let nums = vec![1, 2];
        let expected = vec![vec![], vec![1], vec![2], vec![1, 2]];
        let result = subsets(nums);
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_1_2_3() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let result = subsets(nums);
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_0() {
        let nums = vec![0];
        let expected = vec![vec![], vec![0]];
        let result = subsets(nums);
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_2_1() {
        let nums = vec![2, 1];
        let expected = vec![vec![], vec![1], vec![2], vec![1, 2]];
        let result = subsets(nums);
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }

    #[test]
    fn test_subsets_1_2_3_4() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        let result = subsets(nums);
        assert_eq!(sorted_subsets(result), sorted_subsets(expected));
    }
}
