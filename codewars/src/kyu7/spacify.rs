// https://www.codewars.com/kata/57f8ee485cae443c4d000127/
fn spacify(s: &str) -> String {
    s.chars()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::spacify;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(spacify(s), expected, "{ERR_MSG} with s = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("hello world", "h e l l o   w o r l d");
        dotest("12345", "1 2 3 4 5");
        dotest("Pippi", "P i p p i");
        dotest("a", "a");
        dotest("", "");
    }
}
