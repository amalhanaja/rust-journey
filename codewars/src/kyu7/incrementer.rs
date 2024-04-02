// https://www.codewars.com/kata/590e03aef55cab099a0002e8/
fn incrementer(nums: &[u32]) -> Vec<u32> {
    nums.into_iter()
        .enumerate()
        .map(|(i, n)| ((i + 1) as u32 + n) % 10)
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::incrementer;

    #[test]
    fn sample_tests() {
        assert_eq!(incrementer(&[]), vec![]);
        assert_eq!(incrementer(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(incrementer(&[4, 6, 7, 1, 3]), vec![5, 8, 0, 5, 8]);
        assert_eq!(incrementer(&[3, 6, 9, 8, 9]), vec![4, 8, 2, 2, 4]);
        assert_eq!(
            incrementer(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 8]),
            vec![2, 4, 6, 8, 0, 2, 4, 6, 8, 9, 0, 1, 2, 2]
        );
    }
}
