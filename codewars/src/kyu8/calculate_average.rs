// https://www.codewars.com/kata/57a2013acf1fa5bfc4000921/
fn find_average(slice: &[f64]) -> f64 {
    if slice.len() == 0 {
        return 0 as f64;
    }
    slice.iter().fold(0 as f64, |acc, v| acc + v) / slice.len() as f64
}

#[cfg(test)]
mod tests {
    use super::find_average;
    use float_eq::assert_float_eq;

    #[test]
    fn example() {
        let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];

        assert_float_eq!(
            find_average(&input),
            200.0 / 13.0,
            abs <= 1e-9,
            r2nd <= 4.0 * f64::EPSILON
        );

        assert_float_eq!(
            find_average(&[]),
            0.0,
            abs <= 1e-9,
            r2nd <= 4.0 * f64::EPSILON
        );
    }
}
