
struct Solution();

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::new();
        // init default value -1
        for i in 0..nums.len() {
            v.push(-1);
        }

        for i in 0..nums.len() {
            let j: usize = index[i] as usize;
            if *v.get(j as usize).unwrap() == -1 {
                v[j] = nums[i as usize];
            }else {
                v.insert(j, nums[i as usize]);
            }
        }

        v.truncate(nums.len());
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_create_target_array() {
        let nums = vec![0,1,2,3,4];
        let index = vec![0,1,2,2,1];
        let result = vec![0,4,1,3,2];
        assert_eq!(Solution::create_target_array(nums, index), result);
    }

}