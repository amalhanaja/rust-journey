// https://www.codewars.com/kata/5b39e3772ae7545f650000fc/
fn remove_duplicate_words(s: &str) -> String {
    s.split_ascii_whitespace()
        .fold(String::new(), |acc, current| {
            if acc.contains(current) {
                return acc;
            }
            format!("{} {}", acc, current)
        })
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        assert_eq!(
            remove_duplicate_words(
                "alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"
            ),
            "alpha beta gamma delta"
        );
        assert_eq!(
            remove_duplicate_words("my cat is my cat fat"),
            "my cat is fat"
        );
    }
}
