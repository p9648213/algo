//! https://leetcode.com/problems/combination-sum-iii/

// Find all valid combinations of k numbers that sum up to n such that the following conditions are true:

//     Only numbers 1 through 9 are used.
//     Each number is used at most once.

// Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.

// Example 1:

// Input: k = 3, n = 7
// Output: [[1,2,4]]
// Explanation:
// 1 + 2 + 4 = 7
// There are no other valid combinations.

// Example 2:

// Input: k = 3, n = 9
// Output: [[1,2,6],[1,3,5],[2,3,4]]
// Explanation:
// 1 + 2 + 6 = 9
// 1 + 3 + 5 = 9
// 2 + 3 + 4 = 9
// There are no other valid combinations.

// Example 3:

// Input: k = 4, n = 1
// Output: []
// Explanation: There are no valid combinations.
// Using 4 different numbers in the range [1,9], the smallest sum we can get is 1+2+3+4 = 10 and since 10 > 1, there are no valid combination.

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let max = if n > 9 { 9 } else { n };

    let mut result = vec![];

    helper(n, k, 1, &mut vec![], 0, &mut result, max);

    result
}

pub fn helper(
    n: i32,
    k: i32,
    start: i32,
    curr: &mut Vec<i32>,
    curr_sum: i32,
    result: &mut Vec<Vec<i32>>,
    max: i32,
) {
    if curr.len() as i32 == k && curr_sum == n {
        result.push(curr.clone());
        return;
    }

    for j in start..=max {
        curr.push(j);
        helper(n, k, j + 1, curr, curr_sum + j, result, max);
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

    #[test]
    fn test_combination_sum3_k3_n6() {
        let k = 3;
        let n = 6;
        let expected = vec![vec![1, 2, 3]];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum3_k3_n7() {
        let k = 3;
        let n = 7;
        let expected = vec![vec![1, 2, 4]];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum3_k3_n9() {
        let k = 3;
        let n = 9;
        let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum3_k3_n14() {
        let k = 3;
        let n = 14;
        let expected = vec![
            vec![1, 4, 9],
            vec![1, 5, 8],
            vec![1, 6, 7],
            vec![2, 3, 9],
            vec![2, 4, 8],
            vec![2, 5, 7],
            vec![3, 4, 7],
            vec![3, 5, 6],
        ];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum3_k4_n10() {
        let k = 4;
        let n = 10;
        let expected = vec![vec![1, 2, 3, 4]];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combination_sum3_k4_n30() {
        let k = 4;
        let n = 30;
        let expected = vec![vec![6, 7, 8, 9]];
        let result = combination_sum3(k, n);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }
}
