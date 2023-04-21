fn reverse_words(words: &str) -> String {
    return words.split(' ').rev().collect::<Vec<&str>>().join(" ");
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
        assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}
