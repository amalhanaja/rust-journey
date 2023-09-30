struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut third: i32 = i32::MIN;
        for &num in nums.iter().rev() {
            if num < third {
                return true;
            }
            while let Some(&top) = stack.last() {
                if top < num {
                    third = stack.pop().unwrap();
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::_456_132_pattern::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
