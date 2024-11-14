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

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    for (i, _) in nums.iter().enumerate() {
        let nums_range: Vec<i32> = nums.clone().into_iter().skip(i).collect();
        let mut vec_clone = nums_range.clone();
        println!("{:?}", nums_range);
        for (j, _) in nums_range.iter().enumerate() {
            vec_clone[i] = nums_range[j];
            result.push(vec_clone.clone());
            vec_clone = nums.clone()
        }
    }

    result
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
                [3, 1, 2],
                [3, 2, 1]
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
