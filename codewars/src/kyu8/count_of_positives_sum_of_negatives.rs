// https://www.codewars.com/kata/576bb71bbbcf0951d5000044/
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.len() == 0 {
        return vec![];
    }
    input.iter().fold(vec![0, 0], |acc, &x| {
        let count_positive = acc.get(0).unwrap() + (x.is_positive() as i32);
        let sum_of_negative = match x.is_negative() {
            true => acc.get(1).unwrap() + x,
            false => *acc.get(1).unwrap(),
        };
        vec![count_positive, sum_of_negative]
    })
}

#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;

    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert!(
            actual == expected,
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }
    #[test]
    fn fixed_tests() {
        dotest(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15],
            &[10, -65],
        );
        dotest(&[], &[]);
        dotest(
            &[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14],
            &[8, -50],
        );
        dotest(&[0, 1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[1, 2, 3, 4, 5], &[5, 0]);
        dotest(&[0, -1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[-1, -2, -3, -4, -5], &[0, -15]);
        dotest(&[0, 0, 0, 0], &[0, 0]);
        dotest(&[0], &[0, 0]);
    }
}
