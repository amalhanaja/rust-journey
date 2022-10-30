use std::collections::HashMap;

// https://leetcode.com/problems/roman-to-integer/
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();
        let mut number = 0;
        let mut prev: i32 = 0;
        s.chars().rev().for_each(|c| {
            let current = roman.get(&c).unwrap();
            if *current < prev{
                number -= current;
            } else {
                number += current;
            }
            prev = *current;
        });
        return number;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
