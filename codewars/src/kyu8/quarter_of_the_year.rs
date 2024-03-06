// https://www.codewars.com/kata/5ce9c1000bab0b001134f5af/
fn quarter_of(month: u8) -> u8 {
    (month as f32 / 3f32).ceil() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
        assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
        assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
    }
}
