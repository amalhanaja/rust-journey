// https://www.codewars.com/kata/56a4872cbb65f3a610000026

fn max_rot(n: u64) -> u64 {
    let mut digits = n.to_string().chars().collect::<Vec<char>>();
    let mut max = n;
    for i in 0..digits.len() {
        let mut new_digits = digits[..i].to_vec();
        new_digits.extend(digits[i + 1..].to_vec());
        let removed = digits.remove(i);
        new_digits.push(removed);
        digits = new_digits;
        max = (*digits)
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
            .max(max);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(max_rot(56789), 68957);
        assert_eq!(max_rot(38458215), 85821534);
        assert_eq!(max_rot(195881031), 988103115);
        assert_eq!(max_rot(896219342), 962193428);
        assert_eq!(max_rot(69418307), 94183076);
    }
}
