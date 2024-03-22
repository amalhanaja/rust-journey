// https://www.codewars.com/kata/5a523566b3bfa84c2e00010b/
fn min_sum(xs: &[u64]) -> u64 {
    let mut vec = xs.to_vec();
    let mut result = 0;
    vec.sort();
    while !vec.is_empty() {
        result += vec.remove(0) * vec.remove(vec.len() - 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5, 4, 2, 3]), 22);
        assert_eq!(min_sum(&[12, 6, 10, 26, 3, 24]), 342);
        assert_eq!(min_sum(&[9, 2, 8, 7, 5, 4, 0, 6]), 74);
    }
}
