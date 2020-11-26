/*
初步分析，需要三次嵌套遍历，计算找出合适的三元组值
 */

struct Solution();

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut matched = 0;
        for i in 0..arr.len()-2 {
            for j in i+1..arr.len()-1 {
                // 此处先判断符合条件的，减少内部k的循环次数
                if (arr[i] - arr[j]).abs() <= a {
                    for k in j + 1..arr.len() {
                        if (arr[j] - arr[k]).abs() <= b
                            && (arr[i] - arr[k]).abs() <= c {
                            matched += 1;
                        }
                    }
                }
            }
        }

        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_count_good_triplets() {
        let v = vec![3,0,1,1,9,7];
        assert_eq!(Solution::count_good_triplets(v,7,2,3), 4);
        let v = vec![1,1,2,2,3];
        assert_eq!(Solution::count_good_triplets(v, 0,0,1), 0);
    }
}