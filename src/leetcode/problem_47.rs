// https://leetcode.com/problems/permutations-ii/description/

// Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.

// Example 1:

// Input: nums = [1,1,2]
// Output:
// [[1,1,2],
//  [1,2,1],
//  [2,1,1]]

// Example 2:

// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

use std::collections::HashMap;

// Time: O(n! * n) | S: O(n)
pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums_copy = nums.clone();
    let mut result: Vec<Vec<i32>> = vec![];
    let n = nums.len();

    helper(0, n, &mut result, &mut nums_copy);

    result
}

fn helper(i: usize, length: usize, result: &mut Vec<Vec<i32>>, nums: &mut Vec<i32>) {
    if i == length - 1 {
        result.push(nums.clone());
        return;
    }

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for j in i..length {
        if !hash.contains_key(&nums[j]) {
            hash.insert(nums[j], 1);
            nums.swap(i, j);
            helper(i + 1, length, result, nums);
            nums.swap(i, j);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_permute() {
        let input = vec![3, 3, 2];
        let result = permute_unique(input);
        assert_eq!(result, vec![[3, 3, 2], [3, 2, 3], [2, 3, 3]])
    }

    #[test]
    fn test_unique_permute_2() {
        let input = vec![1, 1, 2];
        let result = permute_unique(input);
        assert_eq!(result, vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]])
    }

    #[test]
    fn test_unique_permute_3() {
        let input = vec![2, 2, 2];
        let result = permute_unique(input);
        assert_eq!(result, vec![[2, 2, 2]])
    }

    #[test]
    fn test_unique_permute_4() {
        let input = vec![0, 1, 0, 1];
        let result = permute_unique(input);
        assert_eq!(
            result,
            vec![
                [0, 1, 0, 1],
                [0, 1, 1, 0],
                [0, 0, 1, 1],
                [1, 0, 0, 1],
                [1, 0, 1, 0],
                [1, 1, 0, 0]
            ]
        )
    }

    #[test]
    fn test_unique_permute_5() {
        let input = vec![1];
        let result = permute_unique(input);
        assert_eq!(result, vec![[1]])
    }
}
