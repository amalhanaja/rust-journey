struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let string_vec: Vec<String> = s
            .split_terminator(' ')
            .map(|word| word.chars().into_iter().rev().collect::<String>())
            .collect();
        return string_vec.join(" ");
    }
}

#[cfg(test)]
mod tests {
    use crate::_557_reverse_words_in_a_string_III::Solution;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );

        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
