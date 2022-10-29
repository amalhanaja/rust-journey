
// https://leetcode.com/problems/palindrome-number/
struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = format!("{}", x);
        let reversed = s.chars().rev().collect::<String>();
        return s == reversed;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn test_case_1() {
        assert_eq!(Solution::is_palindrome(121), true)
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::is_palindrome(-121), false)
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::is_palindrome(10), false)
    }
}