// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/
fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
