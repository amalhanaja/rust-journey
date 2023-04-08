// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

fn persistence(num: u64) -> u64 {
    return persistence_recursive(0, num);
}

fn persistence_recursive(accumulator: u64, num: u64) -> u64 {
    let digits = format!("{}", num);
    if digits.len() == 1 {
        return accumulator;
    }
    let result: u64 = digits
        .chars()
        .map(|c| c.to_digit(10))
        .fold(1, |acc, x| acc * x.unwrap() as u64);
    return persistence_recursive(accumulator + 1, result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
