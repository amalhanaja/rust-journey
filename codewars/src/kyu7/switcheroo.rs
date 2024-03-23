// https://www.codewars.com/kata/57f759bb664021a30300007d/
fn switcheroo(s: &str) -> String {
    s.chars().map(|x| match x {
        'a' => 'b',
        'b' => 'a',
        _ => x,
    }).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(switcheroo("abc"), "bac");
        assert_eq!(switcheroo("aaabcccbaaa"), "bbbacccabbb");
        assert_eq!(switcheroo("ccccc"), "ccccc");
        assert_eq!(switcheroo("abababababababab"), "babababababababa");
        assert_eq!(switcheroo("aaaaa"), "bbbbb");
    }
}
