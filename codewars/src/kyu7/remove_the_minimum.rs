// https://www.codewars.com/kata/563cf89eb4747c5fb100001b/
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.is_empty() {
        return vec![];
    }
    let min = numbers.iter().min().unwrap();
    let removed_index = numbers.iter().position(|x| x == min).unwrap();
    let mut vec = numbers.to_owned();
    vec.remove(removed_index as usize);
    vec
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_smallest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(
            remove_smallest(a),
            expected,
            "{ERR_MSG} with numbers = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
