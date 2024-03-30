fn sort_my_string(s: &str) -> String {
    (0..s.len())
        .fold(["".to_string(), "".to_string()], |acc, current| {
            match current % 2 {
                0 => [
                    format!("{}{}", acc[0], &s[current..current + 1]),
                    format!("{}", acc[1]),
                ],
                _ => [
                    format!("{}", acc[0]),
                    format!("{}{}", acc[1], &s[current..current + 1]),
                ],
            }
        })
        .join(" ")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::sort_my_string;

    fn dotest(s: &str, expected: &str) {
        let actual = sort_my_string(s);
        assert!(
            actual == expected,
            "With s = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        dotest("CodeWars", "CdWr oeas");
        dotest("YCOLUE'VREER", "YOU'RE CLEVER");
    }
}
