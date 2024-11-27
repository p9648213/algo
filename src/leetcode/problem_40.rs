// https://leetcode.com/problems/combination-sum-ii/

// Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

// Each number in candidates may only be used once in the combination.

// Note: The solution set must not contain duplicate combinations.

// Example 1:

// Input: candidates = [10,1,2,7,6,1,5], target = 8
// Output:
// [
// [1,1,6],
// [1,2,5],
// [1,7],
// [2,6]
// ]

// Example 2:

// Input: candidates = [2,5,2,1,2], target = 5
// Output:
// [
// [1,2,2],
// [5]
// ]

use std::collections::HashMap;

// Time O(2^n) | Space: O(n)
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates.clone();
    candidates.sort();

    let mut result = vec![];

    helper(target, &candidates, 0, &mut vec![], 0, &mut result);

    result
}

pub fn helper(
    target: i32,
    candidates: &Vec<i32>,
    index: i32,
    curr: &mut Vec<i32>,
    curr_sum: i32,
    result: &mut Vec<Vec<i32>>,
) {
    if curr_sum > target {
        return;
    }

    if curr_sum == target {
        result.push(curr.clone());
        return;
    }

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for j in index..candidates.len() as i32 {
        if hash.get(&candidates[j as usize]).is_some() {
            continue;
        }

        curr.push(candidates[j as usize]);

        hash.insert(candidates[j as usize], 1);

        helper(
            target,
            candidates,
            j + 1,
            curr,
            curr_sum + candidates[j as usize],
            result,
        );

        curr.pop();
    }
}

pub fn sorted_combinations(combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut combinations = combinations
        .into_iter()
        .map(|mut arr| {
            arr.sort();
            arr
        })
        .collect::<Vec<_>>();
    combinations.sort();
    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted_combinations(mut combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for combination in &mut combinations {
            combination.sort_unstable();
        }
        combinations.sort_unstable();
        combinations
    }

    #[test]
    fn test_combination_sum2_case1() {
        let candidates = vec![3, 5, 2, 1, 3];
        let target = 7;
        let expected = vec![vec![1, 3, 3], vec![5, 2]];
        let result = combination_sum2(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum2_case2() {
        let candidates = vec![2, 3, 2, 7];
        let target = 7;
        let expected = vec![vec![2, 2, 3], vec![7]];
        let result = combination_sum2(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum2_case3() {
        let candidates = vec![1, 1, 1, 1, 2, 2];
        let target = 4;
        let expected = vec![vec![1, 1, 1, 1], vec![1, 1, 2], vec![2, 2]];
        let result = combination_sum2(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum2_case4() {
        let candidates = vec![4, 6, 8, 4];
        let target = 4;
        let expected = vec![vec![4]];
        let result = combination_sum2(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum2_case5() {
        let candidates = vec![10, 20, 30];
        let target = 5;
        let expected: Vec<Vec<i32>> = vec![];
        let result = combination_sum2(candidates, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_case6() {
        let candidates = vec![2, 5, 2, 1, 2];
        let target = 5;
        let expected: Vec<Vec<i32>> = vec![vec![1, 2, 2], vec![5]];
        let result = combination_sum2(candidates, target);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combination_sum2_case7() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let expected: Vec<Vec<i32>> = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
        let result = combination_sum2(candidates, target);
        assert_eq!(result, expected);
    }
}
