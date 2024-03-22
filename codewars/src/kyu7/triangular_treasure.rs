// https://www.codewars.com/kata/525e5a1cb735154b320002c8/
fn triangular(n: i32) -> i32 {
    (1..=n).sum::<i32>().max(0)
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::triangular;

    #[test]
    fn sample_tests() {
        assert_eq!(triangular(2), 3);
        assert_eq!(triangular(4), 10);
        assert_eq!(triangular(9), 45);
        assert_eq!(triangular(-9), 0);
    }
}
