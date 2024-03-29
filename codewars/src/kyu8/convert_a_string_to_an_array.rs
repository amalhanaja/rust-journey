// https://www.codewars.com/kata/57e76bc428d6fbc2d500036d/
fn string_to_array(s: &str) -> Vec<String> {
    s.split(" ").map(|x| x.to_string()).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::string_to_array;

    fn dotest(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert!(
            actual == expected,
            "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("Robin Singh", &["Robin", "Singh"]);
        dotest("CodeWars", &["CodeWars"]);
        dotest(
            "I love arrays they are my favorite",
            &["I", "love", "arrays", "they", "are", "my", "favorite"],
        );
        dotest("1 2 3", &["1", "2", "3"]);
    }
}
