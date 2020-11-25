
struct Solution();

impl Solution {
    /*
    先定义两个标尺，指向开始和结尾。然后找到中间位置，判断开始和中间位置哪个大，如果中间位置大，则临时把结果存储为开始，
    然后继续遍历右侧（中间位置+1与结尾），重复上述；如果开始位置大，则把临时结果存储为中间位置，继续遍历左侧（开始与中间
    位置-1）。时间复杂度O(logn)
     */
    pub fn middle_search(left: usize, right: usize, arr: Vec<i32>, result: &mut i32) {
        if left > right {
            return;
        }

        let middle = (right - left)/2 + left;

        if *result > arr[left] {
            *result = arr[left];
        }

        if arr[left] <= arr[middle] {
            if *result > arr[left] {
                *result = arr[left];
            }
            Solution::middle_search(middle+1, right, arr, result);
        }else{
            if *result > arr[middle] {
                *result = arr[middle];
            }
            Solution::middle_search(left, middle-1, arr, result);
        }
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        // 使用二分查找思想
        let mut res = std::i32::MAX;
        if nums.len() == 0 {
            return 0;
        }
        Solution::middle_search(0, nums.len()-1, nums, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_find_min() {
        let v = vec![3,4,5,1,2];
            assert_eq!(Solution::find_min(v), 1);
        let v = vec![4,5,6,7,0,1,2];
        assert_eq!(Solution::find_min(v), 0);
        let v = vec![11,13,15,17];
        assert_eq!(Solution::find_min(v), 11);
        let v = vec![1,1,1,1,1,0,1];
        assert_eq!(Solution::find_min(v), 0);
    }
}