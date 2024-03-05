// https://www.codewars.com/kata/5aa736a455f906981800360d/
fn feast(beast: &str, dish: &str) -> bool {
    first_and_last(beast) == first_and_last(dish)
}

fn first_and_last(s: &str) -> String {
    s.chars().step_by(s.len() - 1).collect::<String>()
}

// Rust test example:
#[test]
fn sample_test_cases() {
    assert_eq!(feast("great blue heron", "garlic naan"), true);
    assert_eq!(feast("chickadee", "chocolate cake"), true);
    assert_eq!(feast("brown bear", "bear claw"), false);
}
