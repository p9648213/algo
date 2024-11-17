// Coding Exercise Power Sum
// Instructions

// Question:

// Power Sum - Let’s define a peculiar type of array in which each element is either an integer or another peculiar array. Assume that a peculiar array is never empty. Write a function that will take a peculiar array as its input and find the sum of its elements. If an array is an element in the peculiar array you have to convert it to it’s equivalent value so that you can sum it with the other elements. Equivalent value of an array is the sum of its elements raised to the number which represents how far nested it is.

// example - [2,3,[4,1,2],1] = 2+3+ (4+1+2)^2 + 1
// example - [1,2,[7,[3,4],2]] = 1 + 2 + (7+(3+4)^3+2)^2

#[derive(Debug)]
pub enum PeculiarArray {
    Integer(i32),
    Array(Vec<PeculiarArray>),
}

// Time: O(N) (N is number of element in array and sub array)
// Space: O(d) (D is the deep of the array)
pub fn power_sum(n: &[PeculiarArray]) -> i32 {
    // calc_power_sum(n, 1)
    calc_power_sum_2(n, 1)
}

pub fn calc_power_sum(n: &[PeculiarArray], mut level: u32) -> i32 {
    let mut result = 0;
    for (index, num) in n.iter().enumerate() {
        match num {
            PeculiarArray::Integer(el) => {
                result = result + el;
                if index == n.len() - 1 {
                    result = result.pow(level);
                }
            }
            PeculiarArray::Array(vec) => {
                level = level + 1;
                result = result + calc_power_sum(vec, level);
                level = level - 1;
                if index == n.len() - 1 {
                    result = result.pow(level);
                }
            }
        }
    }
    result
}

fn calc_power_sum_2(n: &[PeculiarArray], level: u32) -> i32 {
    let mut sum = 0;
    for num in n {
        match num {
            PeculiarArray::Integer(el) => {
                sum = sum + el;
            }
            PeculiarArray::Array(vec) => sum = sum + calc_power_sum_2(vec, level + 1),
        }
    }
    sum.pow(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_sum_nested_power_sum() {
        let input = vec![
            PeculiarArray::Integer(1),
            PeculiarArray::Integer(5),
            PeculiarArray::Array(vec![
                PeculiarArray::Integer(3),
                PeculiarArray::Integer(4),
                PeculiarArray::Array(vec![PeculiarArray::Integer(5), PeculiarArray::Integer(6)]),
            ]),
        ];
        let result = power_sum(&input);
        let expected = 1 + 5 + (3 + 4 + (5_usize + 6).pow(3)).pow(2);
        assert_eq!(result, expected as i32);
        println!("Input: [1, 5, [3, 4, [5, 6]]] | Expected Calculation: 1 + 5 + (3 + 4 + (5 + 6)^3)^2 = {} | Result: {}", expected, result);
    }

    #[test]
    fn test_power_sum_simple_nested_array() {
        let input = vec![
            PeculiarArray::Integer(2),
            PeculiarArray::Integer(3),
            PeculiarArray::Array(vec![
                PeculiarArray::Integer(4),
                PeculiarArray::Integer(1),
                PeculiarArray::Integer(2),
            ]),
            PeculiarArray::Integer(1),
        ];
        let result = power_sum(&input);
        let expected = 2 + 3 + (4_usize + 1 + 2).pow(2) + 1;
        assert_eq!(result, expected as i32);
        println!("Input: [2, 3, [4, 1, 2], 1] | Expected Calculation: 2 + 3 + (4 + 1 + 2)^2 + 1 = {} | Result: {}", expected, result);
    }

    #[test]
    fn test_power_sum_multi_level_nesting() {
        let input = vec![
            PeculiarArray::Integer(1),
            PeculiarArray::Integer(2),
            PeculiarArray::Array(vec![
                PeculiarArray::Integer(7),
                PeculiarArray::Array(vec![PeculiarArray::Integer(3), PeculiarArray::Integer(4)]),
                PeculiarArray::Integer(2),
            ]),
        ];
        let result = power_sum(&input);
        let expected = 1 + 2 + (7 + (3_usize + 4).pow(3) + 2).pow(2);
        assert_eq!(result, expected as i32);
        println!("Input: [1, 2, [7, [3, 4], 2]] | Expected Calculation: 1 + 2 + (7 + (3 + 4)^3 + 2)^2 = {} | Result: {}", expected, result);
    }

    #[test]
    fn test_power_sum_single_element() {
        let input = vec![PeculiarArray::Integer(1)];
        let result = power_sum(&input);
        let expected = 1;
        assert_eq!(result, expected);
        println!(
            "Input: [1] | Expected Calculation: 1 = {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_power_sum_single_nested_array() {
        let input = vec![PeculiarArray::Array(vec![
            PeculiarArray::Integer(1),
            PeculiarArray::Integer(2),
            PeculiarArray::Integer(3),
        ])];
        let result = power_sum(&input);
        let expected = (1_usize + 2 + 3).pow(2);
        assert_eq!(result, expected as i32);
        println!(
            "Input: [[1, 2, 3]] | Expected Calculation: (1 + 2 + 3)^2 = {} | Result: {}",
            expected, result
        );
    }

    #[test]
    fn test_power_sum_complex_nesting() {
        let input = vec![
            PeculiarArray::Integer(1),
            PeculiarArray::Array(vec![
                PeculiarArray::Integer(2),
                PeculiarArray::Array(vec![PeculiarArray::Integer(3), PeculiarArray::Integer(4)]),
            ]),
        ];
        let result = power_sum(&input);
        let expected = 1 + (2 + (3_usize + 4).pow(3)).pow(2);
        assert_eq!(result, expected as i32);
        println!("Input: [1, [2, [3, 4]]] | Expected Calculation: 1 + (2 + (3 + 4)^3)^2 = {} | Result: {}", expected, result);
    }
}
