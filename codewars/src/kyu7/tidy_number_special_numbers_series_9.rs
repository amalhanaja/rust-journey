// https://www.codewars.com/kata/5a87449ab1710171300000fd/
fn tidy_number(n: u64) -> bool {
    let mut prev = n.to_string().chars().nth(0).unwrap();
    for c in n.to_string().chars().skip(1) {
        match prev.cmp(&c) {
            std::cmp::Ordering::Greater => return false,
            _ => prev = c,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }
}
