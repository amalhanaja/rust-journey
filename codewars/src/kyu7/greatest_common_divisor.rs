fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(gcd(30, 12), 6);
        assert_eq!(gcd(8, 9), 1);
        assert_eq!(gcd(1, 1), 1);
    }
}
