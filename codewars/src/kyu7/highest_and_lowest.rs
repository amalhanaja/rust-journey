// https://www.codewars.com/kata/554b4ac871d6813a03000035/
fn high_and_low(numbers: &str) -> String {
    let vec = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    format!("{} {}", vec.iter().max().unwrap(), vec.iter().min().unwrap())
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
