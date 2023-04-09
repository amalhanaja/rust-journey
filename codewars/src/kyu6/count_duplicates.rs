// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/

use std::collections::{HashMap};

fn count_duplicates(text: &str) -> u32 {
    if text == "" {
        return 0
    }
    let v: Vec<i32> = text.chars().fold(HashMap::new(), |mut acc, x| {
        let normalize_char: char = x.to_ascii_lowercase();
        let count = acc.get(&normalize_char).unwrap_or(&0).to_owned() + 1;
        acc.insert(normalize_char, count);
        acc
    }).into_values().filter(|v| v.to_owned() > 1).collect();
    return v.len() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }

    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }

    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}
