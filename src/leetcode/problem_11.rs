// https://leetcode.com/problems/container-with-most-water/description/

// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.

// Example 1:

// Input: height = [1,8,6,2,5,4,8,3,7]
// Output: 49
// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.

// Example 2:

// Input: height = [1,1]
// Output: 1

// Time: O(n^2) | Space: O(1)
pub fn max_area2(height: Vec<i32>) -> i32 {
    if height.len() <= 1 {
        return 0;
    }

    let mut i = 0;
    let mut j = 1;

    let mut result = 0;

    while i <= height.len() - 2 {
        while j < height.len() {
            if height[i] > height[j] {
                if height[j] as usize * (j - i) > result {
                    result = height[j] as usize * (j - i);
                }
            } else if height[i] as usize * (j - i) > result {
                result = height[i] as usize * (j - i);
            }
            j += 1
        }
        i += 1;
        j = i + 1;
    }

    result as i32
}

// Time: O(n) | Space: O(1)
pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() <= 1 {
        return 0;
    }

    let mut i = 0;
    let mut j = height.len() - 1;

    let mut result_area = std::cmp::min(height[i], height[j]) as usize * (j - i);

    while i != j {
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }

        if std::cmp::min(height[i], height[j]) as usize * (j - i) > result_area {
            result_area = std::cmp::min(height[i], height[j]) as usize * (j - i)
        }
    }

    result_area as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        assert_eq!(max_area(vec![]), 0);
    }

    #[test]
    fn test_one_element() {
        assert_eq!(max_area(vec![5]), 0);
    }

    #[test]
    fn test_heights_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_heights_2() {
        assert_eq!(max_area(vec![1, 1, 1, 1, 1, 3]), 5);
    }

    #[test]
    fn test_heights_3() {
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn test_heights_4() {
        assert_eq!(max_area(vec![1, 2, 1]), 2);
    }
}
