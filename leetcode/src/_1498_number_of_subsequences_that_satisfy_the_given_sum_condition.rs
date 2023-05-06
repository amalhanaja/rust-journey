// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/

struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut result = 0;
        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        let modulo = 1e9 as i32 + 7;
        let mut pows: Vec<i32> = vec![0; n];
        println!("Capacity: {}", pows.len());
        pows[0] = 1;
        for i in 1..n {
            pows[i] =  pows[i - 1] * 2 % modulo
        }
        while left <= right {
            if nums[left] + nums[right] > target {
                right -= 1;
            } else {
                result = (result + pows[right - left]) % modulo;
                left += 1;
            }
        }
        return result
    }
}

#[cfg(test)]
mod tests {
    use crate::_1498_number_of_subsequences_that_satisfy_the_given_sum_condition::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::num_subseq(vec![3,5,6,7], 9), 4);
        assert_eq!(Solution::num_subseq(vec![3,3,6,8], 10), 6);
        assert_eq!(Solution::num_subseq(vec![2,3,3,4,6,7], 12), 61);
    }
}