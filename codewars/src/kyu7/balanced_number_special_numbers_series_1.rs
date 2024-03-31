// https://www.codewars.com/kata/5a4e3782880385ba68000018/
fn balanced_num(n: u64) -> String {
    let n = n.to_string();
    if n.len() <= 1 {
        return "Balanced".to_string();
    }
    let mid = n.len() / 2;
    let mid_range = match n.len() % 2 {
        0 => (mid - 1)..(mid + 1),
        _ => mid..mid + 1,
    };
    let left = n[0..mid_range.start]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    let right = n[mid_range.end..]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    if left == right {
        return "Balanced".to_string();
    }
    "Not Balanced".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }

    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");
    }
}
