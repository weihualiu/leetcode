/*
Given an array of positive integers arr, calculate the sum of all possible odd-length subarrays.

A subarray is a contiguous subsequence of the array.

Return the sum of all odd-length subarrays of arr.



Example 1:

Input: arr = [1,4,2,5,3]
Output: 58
Explanation: The odd-length subarrays of arr and their sums are:
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
If we add all these together we get 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
Example 2:

Input: arr = [1,2]
Output: 3
Explanation: There are only 2 subarrays of odd length, [1] and [2]. Their sum is 3.
Example 3:

Input: arr = [10,11,12]
Output: 66


Constraints:

1 <= arr.length <= 100
1 <= arr[i] <= 1000
 */
/// odd number 奇数
/// even number 偶数

struct Solution();

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let step = 2;
        let odd_init = 3;
        for i in 0..arr.len() {
            sum = sum + arr[i];
            let mut j = i;
            while j + odd_init <= arr.len() {
                for k in i..j+odd_init {
                    sum = sum + arr[k];
                }
                j += step;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_sum_odd_length_subarrays() {
        let v = vec![1,4,2,5,3];
        assert_eq!(Solution::sum_odd_length_subarrays(v), 58);
        let v = vec![1,2];
        assert_eq!(Solution::sum_odd_length_subarrays(v), 3);
        let v = vec![10,11,12];
        assert_eq!(Solution::sum_odd_length_subarrays(v), 66);
    }
}