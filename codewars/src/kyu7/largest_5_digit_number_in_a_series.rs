// https://www.codewars.com/kata/51675d17e0c1bed195000001/
fn largest_five_digit_number(num: &str) -> u32 {
    num.chars()
        .collect::<Vec<_>>()
        .windows(5)
        .map(|chars| {
            chars
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u32>()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
        assert_eq!(largest_five_digit_number(&"731674765"), 74765);
    }
}
