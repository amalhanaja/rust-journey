// https://www.codewars.com/kata/534ea96ebb17181947000ada/

fn break_chocolate(n: u32, m: u32) -> u64 {
    (m.max(1) as u64 * n.max(1) as u64) - 1
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::break_chocolate;

    fn dotest(n: u32, m: u32, expected: u64) {
        let actual = break_chocolate(n, m);
        assert!(
            actual == expected,
            "With n = {n}, m = {m}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(5, 5, 24);
        dotest(7, 4, 27);
        dotest(1, 1, 0);
        dotest(0, 0, 0);
        dotest(6, 1, 5);
    }
}
