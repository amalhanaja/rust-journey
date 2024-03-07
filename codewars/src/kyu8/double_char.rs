// https://www.codewars.com/kata/56b1f01c247c01db92000076/train/rust
fn double_char(s: &str) -> String {
    s.chars().map(|c| format!("{}{}", c, c)).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::double_char;

    fn dotest(s: &str, expected: &str) {
        let actual = double_char(s);
        assert!(
            actual == expected,
            "With s = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn test_hello_world() {
        dotest("Hello World", "HHeelllloo  WWoorrlldd")
    }
    #[test]
    fn test_numbers() {
        dotest("1234!_ ", "11223344!!__  ")
    }
}
