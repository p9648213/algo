//! https://leetcode.com/problems/permutations/description/
// Given an array nums of distinct integers, return all the possible
// permutations
// . You can return the answer in any order.

// Example 1:

// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

// Example 2:

// Input: nums = [0,1]
// Output: [[0,1],[1,0]]

// Example 3:

// Input: nums = [1]
// Output: [[1]]

// Time: O(n! * n) | S: O(n)
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
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

    for j in i..length {
        nums.swap(i, j);
        helper(i + 1, length, result, nums);
        nums.swap(i, j);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permute() {
        let input = vec![1, 2, 3];
        let result = permute(input);
        assert_eq!(
            result,
            vec![
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 2, 1],
                [3, 1, 2]
            ]
        )
    }

    #[test]
    fn test_permute_2() {
        let input = vec![0, 1];
        let result = permute(input);
        assert_eq!(result, vec![[0, 1], [1, 0]])
    }

    #[test]
    fn test_permute_3() {
        let input = vec![1];
        let result = permute(input);
        assert_eq!(result, vec![[1]])
    }
}
