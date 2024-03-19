// https://www.codewars.com/kata/511f11d355fe575d2c000001/
fn two_oldest_ages(ages: &[u8]) -> [u8; 2] {
    let mut vec = ages.to_vec();
    vec.sort();
    vec.reverse();
    [vec[1], vec[0]]
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_oldest_ages() {
        assert_eq!(two_oldest_ages(&[1, 5, 87, 45, 8, 8]), [45, 87]);
        assert_eq!(two_oldest_ages(&[6, 5, 83, 5, 3, 18]), [18, 83]);
        assert_eq!(two_oldest_ages(&[10, 1]), [1, 10]);
        assert_eq!(two_oldest_ages(&[1, 3, 10, 0]), [3, 10]);
    }
}
