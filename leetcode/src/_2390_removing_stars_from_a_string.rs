struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        return s.chars().fold(String::new(), |mut acc, c| {
            if c == '*' {
                acc.pop();
            } else {
                acc.push(c);
            }
            acc
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::_2390_removing_stars_from_a_string::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
        assert_eq!(Solution::remove_stars("erase*****".to_string()), "".to_string());
    }
}
