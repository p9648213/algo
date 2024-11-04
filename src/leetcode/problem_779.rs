//! https://leetcode.com/problems/k-th-symbol-in-grammar/description/

// We build a table of n rows (1-indexed). We start by writing 0 in the 1st row. Now in every subsequent row, we look at the previous row and replace each occurrence of 0 with 01, and each occurrence of 1 with 10.

// For example, for n = 3, the 1st row is 0, the 2nd row is 01, and the 3rd row is 0110.

// Given two integer n and k, return the kth (1-indexed) symbol in the nth row of a table of n rows.

// Row 1: 0
// Row 2: 01 (Row 1's 0 turns into 01)
// Row 3: 0110 (Row 2's 0 becomes 01 and 1 becomes 10)
// Row 4: 01101001

//        0
//    0       1
//  0   1   1   0
// 0 1 1 0 1 0 0 1
// ---------------
// 1 2 3 4 5 6 7 8

//? Example 1:

// Input: n = 1, k = 1
// Output: 0
// Explanation: row 1: 0

//? Example 2:

// Input: n = 2, k = 1
// Output: 0
// Explanation:
// row 1: 0
// row 2: 01

//? Example 3:

// Input: n = 2, k = 2
// Output: 1
// Explanation:
// row 1: 0
// row 2: 01

pub fn kth_grammar(n: i32, k: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_779() {
        let result = kth_grammar(1, 1);
        assert_eq!(result, 0);

        let result = kth_grammar(2, 1);
        assert_eq!(result, 0);

        let result = kth_grammar(2, 2);
        assert_eq!(result, 1);
    }
}
