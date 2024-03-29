// https://www.codewars.com/kata/555eded1ad94b00403000071/
fn series_sum(n: u32) -> String {
    format!(
        "{:.2}",
        (0..n).map(|x| 1. / (1. + (x as f32 * 3.))).sum::<f32>()
    )
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::series_sum;

    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(
            actual == expected,
            "Expected series_sum({input}) to be {expected}, but was {actual}"
        );
    }

    #[test]
    fn sample_tests() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(7, "1.68");
        test(39, "2.26");
        test(0, "0.00");
    }
}
