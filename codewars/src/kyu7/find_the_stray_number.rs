use std::collections::HashSet;

// https://www.codewars.com/kata/57f609022f4d534f05000024/
fn stray(arr: &[u32]) -> u32 {
    let set: HashSet<u32> = HashSet::from_iter(arr.iter().cloned());
    for x in set.iter() {
        if arr.iter().filter(|&n| n == x).count() == 1 {
            return *x;
        }
    }
    0
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::stray;

    fn dotest(arr: &[u32], expected: u32) {
        let actual = stray(arr);
        assert!(
            actual == expected,
            "With arr = {arr:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 1, 1, 1, 1, 1, 2], 2);
        dotest(&[2, 3, 2, 2, 2], 3);
        dotest(&[3, 2, 2, 2, 2], 3);
    }
}
