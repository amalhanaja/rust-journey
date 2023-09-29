struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        // Direction(0=unitialized, 1=increasing,-1=decreasing)
        if nums.len() < 2 {
            return true;
        }
        let mut direction = 0;
        for i in 1..nums.len() {
            let prev = nums[i - 1];
            let current = nums[i];
            if prev > current {
                if direction == 0 {
                    direction = 1;
                } else if direction == -1 {
                    return false;
                }
            } else if prev < current {
                if direction == 0 {
                    direction = -1;
                } else if direction == 1 {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::_896_monotonic_array::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
        assert_eq!(Solution::is_monotonic(vec![6, 5, 4, 4]), true);
        assert_eq!(Solution::is_monotonic(vec![1, 3, 2]), false);
    }
}
