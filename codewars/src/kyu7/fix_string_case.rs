// https://www.codewars.com/kata/5b180e9fedaa564a7000009a/
fn solve(s: &str) -> String {
    let uppercase_count = s.chars().filter(|c| c.is_uppercase()).count();
    let lowercase_count = s.len() - uppercase_count;
    match uppercase_count.cmp(&lowercase_count) {
        std::cmp::Ordering::Greater => s.to_uppercase(),
        _ => s.to_lowercase(),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
