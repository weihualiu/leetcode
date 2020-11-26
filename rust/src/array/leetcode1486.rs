/*
Given an integer n and an integer start.
Define an array nums where nums[i] = start + 2*i (0-indexed) and n == nums.length.
Return the bitwise XOR of all elements of nums.

Example 1:
Input: n = 5, start = 0
Output: 8
Explanation: Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
Where "^" corresponds to bitwise XOR operator.
Example 2:
Input: n = 4, start = 3
Output: 8
Explanation: Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.
Example 3:
Input: n = 1, start = 7
Output: 7
Example 4:
Input: n = 10, start = 5
Output: 2

Constraints:
1 <= n <= 1000
0 <= start <= 1000
n == nums.length
 */

struct Solution();

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut result = start;
        for i in 1..n {
            result = result ^ (start + 2 * i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_xor_operation() {
        let n = 5;
        let statr = 0;
        let result = 8;
        assert_eq!(Solution::xor_operation(n, statr), result);
    }
}