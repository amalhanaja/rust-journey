// https://www.codewars.com/kata/56269eb78ad2e4ced1000013

fn find_next_square(sq: u64) -> Option<u64> {
    let square_root = f64::sqrt(sq as f64);
    let square_root_u64 = square_root.clone() as u64;
    if square_root - square_root_u64 as f64 != 0 as f64 {
        return None
    }
    return Some((square_root_u64.clone() + 1) .pow(2));
}

#[cfg(test)]
mod tests {
    use super::find_next_square;

    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}
