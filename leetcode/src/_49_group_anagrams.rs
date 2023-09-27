use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        strs.iter().for_each(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let sorted_word: String = chars.iter().collect();
            map.entry(sorted_word).or_insert(vec![]).push(word.clone())
        });
        return map.into_values().collect();
    }
}
