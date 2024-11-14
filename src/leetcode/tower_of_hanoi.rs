//! Coding Exercise(Tower of Hanoi)
// Instructions

// The tower of Hanoi is a famous puzzle where we have three rods and N disks. The objective of the puzzle is to move the entire stack to another rod. You are given the number of discs N. Initially, these discs are in the rod 1. You need to print all the steps of discs movement so that all the discs reach the 3rd rod. Also, you need to find the total moves.

// Note: The discs are arranged such that the top disc is numbered 1 and the bottom-most disc is numbered N. Also, all the discs have different sizes and a bigger disc cannot be put on the top of a smaller disc. Refer the provided link to get a better clarity about the puzzle.

//? EXAMPLE:
// For Input (N = 2)
// Output:
// move disk 1 from rod 1 to rod 2
// move disk 2 from rod 1 to rod 3
// move disk 1 from rod 2 to rod 3
// 3

fn toh(n: u32, from: u32, to: u32, aux: u32) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toh_n1() {
        let expected = 1;
        let result = toh(1, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=1, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_toh_n2() {
        let expected = 3;
        let result = toh(2, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=2, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_toh_n3() {
        let expected = 7;
        let result = toh(3, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=3, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_toh_n4() {
        let expected = 15;
        let result = toh(4, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=4, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_toh_n5() {
        let expected = 31;
        let result = toh(5, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=5, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_toh_n6() {
        let expected = 63;
        let result = toh(6, 1, 3, 2);
        assert_eq!(
            result, expected,
            "Input: (N=6, from=1, to=3, aux=2) | Expected: {} | Result: {}",
            expected, result
        );
    }
}
