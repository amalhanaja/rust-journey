// https://www.codewars.com/kata/54ff3102c1bad923760001f3/
fn get_count(string: &str) -> usize {
    string
        .chars()
        .into_iter()
        .filter(|&c| "aiueo".contains(c))
        .count()
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
