fn solution(phrase: &str) -> String {
    return phrase.chars().rev().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
        assert_eq!(solution("AlfiaN"), "NaiflA");
    }
}
