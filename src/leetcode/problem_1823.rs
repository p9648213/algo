//! https://leetcode.com/problems/find-the-winner-of-the-circular-game/
// There are n friends that are playing a game. The friends are sitting in a circle and are numbered from 1 to n in clockwise order. More formally, moving clockwise from the ith friend brings you to the (i+1)th friend for 1 <= i < n, and moving clockwise from the nth friend brings you to the 1st friend.

// The rules of the game are as follows:

// Start at the 1st friend.
// Count the next k friends in the clockwise direction including the friend you started at. The counting wraps around the circle and may count some friends more than once.
// The last friend you counted leaves the circle and loses the game.
// If there is still more than one friend in the circle, go back to step 2 starting from the friend immediately clockwise of the friend who just lost and repeat.
// Else, the last friend in the circle wins the game.

// Given the number of friends, n, and an integer k, return the winner of the game.

//? Example 1:

// Input: n = 5, k = 2
// Output: 3
// Explanation: Here are the steps of the game:
// 1) Start at friend 1.
// 2) Count 2 friends clockwise, which are friends 1 and 2.
// 3) Friend 2 leaves the circle. Next start is friend 3.
// 4) Count 2 friends clockwise, which are friends 3 and 4.
// 5) Friend 4 leaves the circle. Next start is friend 5.
// 6) Count 2 friends clockwise, which are friends 5 and 1.
// 7) Friend 1 leaves the circle. Next start is friend 3.
// 8) Count 2 friends clockwise, which are friends 3 and 5.
// 9) Friend 5 leaves the circle. Only friend 3 is left, so they are the winner.

//? Example 2:

// Input: n = 6, k = 5
// Output: 1
// Explanation: The friends leave in this order: 5, 4, 6, 2, 3. The winner is friend 1.

//? Time: O(n^2) | Space: O(n)
pub fn find_the_winner(n: i32, k: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut player: Vec<i32> = (1..=n).collect();
    let mut count = 0;
    let mut index = 0;

    while player.len() > 1 {
        if count == k - 1 {
            if index as usize > player.len() - 1 {
                index = index as usize % player.len();
            }
            player.remove(index);
            count = 0;
        } else {
            count = count + 1;
            index = index + 1;
        }
    }

    player[0]
}

//? Time: O(n) | Space: O(1)
pub fn find_the_winner_2(n: i32, k: i32) -> i32 {
    let mut survivor = 0;
    for pos in 2..=n {
        survivor = (survivor + k) % pos as i32;
    }
    survivor + 1
}

//? Time: O(n^2) | Space: O(n)
pub fn find_the_winner_recursive(n: i32, k: i32) -> i32 {
    let player: Vec<i32> = (1..=n).collect();
    winner(player, 0, k)
}

pub fn winner(mut array: Vec<i32>, start_index: i32, k: i32) -> i32 {
    if array.len() == 1 {
        return array[0];
    }
    let remove_index = (start_index + k - 1) as usize % array.len();
    array.remove(remove_index);
    winner(array, remove_index as i32, k)
}

//? Time: O(n) | Space: O(n)
pub fn find_the_winner_recursive_2(n: i32, k: i32) -> i32 {
    winner_2(n, k) + 1
}

pub fn winner_2(n: i32, k: i32) -> i32 {
    if n == 1 {
        0
    } else {
        (winner_2(n - 1, k) + k) % n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1823() {
        let result = find_the_winner_2(1, 1);
        assert_eq!(result, 1);

        let result = find_the_winner_2(5, 2);
        assert_eq!(result, 3);

        let result = find_the_winner_2(6, 5);
        assert_eq!(result, 1);

        let result = find_the_winner_2(1, 1);
        assert_eq!(result, 1);

        let result = find_the_winner(5, 2);
        assert_eq!(result, 3);

        let result = find_the_winner(6, 5);
        assert_eq!(result, 1);

        let result = find_the_winner_recursive(1, 1);
        assert_eq!(result, 1);

        let result = find_the_winner_recursive(5, 2);
        assert_eq!(result, 3);

        let result = find_the_winner_recursive(6, 5);
        assert_eq!(result, 1);

        let result = find_the_winner_recursive_2(1, 1);
        assert_eq!(result, 1);

        let result = find_the_winner_recursive_2(5, 2);
        assert_eq!(result, 3);

        let result = find_the_winner_recursive_2(6, 5);
        assert_eq!(result, 1);

        let result = find_the_winner_recursive_2(4, 1);
        assert_eq!(result, 4);
    }
}
