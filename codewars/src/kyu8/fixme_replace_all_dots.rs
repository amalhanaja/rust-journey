// https://www.codewars.com/kata/596c6eb85b0f515834000049/
fn replace_dots(s: &str) -> String {
    s.replace('.', "-")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::replace_dots;

    #[test]
    fn sample_tests() {
        assert_eq!(replace_dots(""), "");
        assert_eq!(replace_dots("no dots"), "no dots");
        assert_eq!(replace_dots("one.two.three"), "one-two-three");
        assert_eq!(replace_dots("........"), "--------");
    }
}
