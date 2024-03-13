// https://www.codewars.com/kata/56747fd5cb988479af000028/
fn get_middle(s: &str) -> &str {
    let mid = s.len() / 2;
    &s[mid-((s.len() % 2 == 0) as usize)..=mid]
}

#[test]
fn example_tests() {
    assert_eq!(get_middle("test"), "es");
    assert_eq!(get_middle("testing"), "t");
    assert_eq!(get_middle("middle"), "dd");
    assert_eq!(get_middle("A"), "A");
    assert_eq!(get_middle("of"), "of");
}
