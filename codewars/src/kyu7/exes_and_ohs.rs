use std::str::Chars;

// https://www.codewars.com/kata/55908aad6620c066bc00002a/
fn xo(string: &'static str) -> bool {
    count_char(string.chars(), 'x') == count_char(string.chars(), 'o')
}

fn count_char(char_vec: Chars, c: char) -> usize {
    char_vec
        .into_iter()
        .filter(|&item| item.to_ascii_lowercase() == c.to_ascii_lowercase())
        .count()
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
