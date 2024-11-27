// https://leetcode.com/problems/combination-sum/description/

// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
// frequency
// of at least one of the chosen numbers is different.

// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

// Example 1:

// Input: candidates = [2,3,6,7], target = 7
// Output: [[2,2,3],[7]]
// Explanation:
// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations.

// Example 2:

// Input: candidates = [2,3,5], target = 8
// Output: [[2,2,2,2],[2,3,3],[3,5]]

// Example 3:

// Input: candidates = [2], target = 1
// Output: []

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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

    for j in index..candidates.len() as i32 {
        curr.push(candidates[j as usize]);
        helper(
            target,
            candidates,
            j,
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
        for combo in &mut combinations {
            combo.sort_unstable();
        }
        combinations.sort_unstable();
        combinations
    }

    #[test]
    fn test_combination_sum_case_1() {
        let candidates = vec![2, 3, 8, 9];
        let target = 9;
        let expected = vec![vec![2, 2, 2, 3], vec![3, 3, 3], vec![9]];
        let result = combination_sum(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum_case_2() {
        let candidates = vec![2, 3, 6];
        let target = 6;
        let expected = vec![vec![2, 2, 2], vec![3, 3], vec![6]];
        let result = combination_sum(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum_case_3() {
        let candidates = vec![2, 3, 7];
        let target = 7;
        let expected = vec![vec![2, 2, 3], vec![7]];
        let result = combination_sum(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum_case_4() {
        let candidates = vec![2, 4];
        let target = 4;
        let expected = vec![vec![2, 2], vec![4]];
        let result = combination_sum(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum_case_5() {
        let candidates = vec![1, 2];
        let target = 3;
        let expected = vec![vec![1, 1, 1], vec![1, 2]];
        let result = combination_sum(candidates, target);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }
}
