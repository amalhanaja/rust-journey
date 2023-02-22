// https://leetcode.com/problems/valid-parentheses/

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut openend_tags_stack: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                '(' | '{' | '[' => openend_tags_stack.push(c.clone()),
                _ => {
                    if openend_tags_stack.is_empty()
                        || !Solution::is_valid_parantheses(openend_tags_stack.pop().unwrap(), c)
                    {
                        return false;
                    }
                }
            }
        }
        return openend_tags_stack.is_empty();
    }

    fn is_valid_parantheses(open_tag: char, close_tag: char) -> bool {
        return match open_tag {
            '(' => close_tag == ')',
            '{' => close_tag == '}',
            '[' => close_tag == ']',
            _ => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        // When
        let result = Solution::is_valid("()".to_string());

        // Then
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_2() {
        // When
        let result = Solution::is_valid("()[]{}".to_string());

        // Then
        assert_eq!(true, result)
    }

    #[test]
    fn test_case_3() {
        // When
        let result = Solution::is_valid("(]".to_string());

        // Then
        assert_eq!(false, result)
    }

    #[test]
    fn test_case_4() {
        // When
        let result = Solution::is_valid("({[()]})".to_string());

        // Then
        assert_eq!(true, result)
    }
}
