// https://www.codewars.com/kata/5a4d303f880385399b000001/
fn strong(n: u64) -> String {
    if n.to_string()
        .chars()
        .map(|c| factorial(c.to_digit(10).unwrap() as u64))
        .sum::<u64>()
        == n
    {
        return "STRONG!!!!".to_string();
    }
    "Not Strong !!".to_string()
}

fn factorial(n: u64) -> u64 {
    (1u64..=n).reduce(|acc, x| acc * x).unwrap_or(1)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");

        // Testing for 40585
        assert_eq!(strong(40585), "STRONG!!!!");
    }
}
