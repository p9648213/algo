// https://leetcode.com/problems/combinations/description/

// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

// You may return the answer in any order.

// Example 1:

// Input: n = 4, k = 2
// Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
// Explanation: There are 4 choose 2 = 6 total combinations.
// Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.

// Example 2:

// Input: n = 1, k = 1
// Output: [[1]]
// Explanation: There is 1 choose 1 = 1 total combination.

// Time: K x nCk | Space: O(k)
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];

    helper(n, k, 1, &mut vec![], &mut result);

    result
}

pub fn helper(n: i32, k: i32, start: i32, curr: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if curr.len() as i32 == k {
        result.push(curr.clone());
        return;
    }

    for j in start..=n {
        curr.push(j);
        helper(n, k, j + 1, curr, result);
        curr.pop();
    }
}

fn sorted_combinations(combinations: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
    fn test_combine_case_1() {
        let n = 4;
        let k = 2;
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let result = combine(n, k);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combine_case_2() {
        let n = 4;
        let k = 3;
        let expected = vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]];
        let result = combine(n, k);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combine_case_3() {
        let n = 4;
        let k = 4;
        let expected = vec![vec![1, 2, 3, 4]];
        let result = combine(n, k);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combine_case_4() {
        let n = 4;
        let k = 1;
        let expected = vec![vec![1], vec![2], vec![3], vec![4]];
        let result = combine(n, k);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }

    #[test]
    fn test_combine_case_5() {
        let n = 5;
        let k = 2;
        let expected = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
            vec![3, 4],
            vec![3, 5],
            vec![4, 5],
        ];
        let result = combine(n, k);
        assert_eq!(sorted_combinations(result), sorted_combinations(expected));
    }
}
