// https://www.codewars.com/kata/57f780909f7e8e3183000078/
fn grow(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|a, b| a * b).unwrap()
}

#[test]
fn basic_test() {
    assert_eq!(grow(vec![1, 2, 3]), 6);
    assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
    assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
}
