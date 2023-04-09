// https://leetcode.com/problems/maximum-gap/
struct Solution;
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        return nums.iter().enumerate().fold(0, |acc, (index, n)| {
            if index == 0 {
                return acc
            }
            let prev = nums.get(index - 1).unwrap();
            let diff = n - prev;
            return acc.max(diff)
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::_164_maximum_gap::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::maximum_gap(vec![3,6,9,1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
        assert_eq!(Solution::maximum_gap(vec![1,2,5,10]), 5);
    }
}