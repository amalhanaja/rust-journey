use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        s.chars().for_each(|c| *char_map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *char_map.entry(c).or_insert(0) -= 1);
        return char_map.into_values().all(|count| count == 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::_242_valid_anagram::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
        assert_eq!(
            Solution::is_anagram("car".to_string(), "rat".to_string()),
            false
        );
    }
}
