// https://www.codewars.com/kata/55902c5eaa8069a5b4000083
fn format_money(amount: f64) -> String {
    format!("${:.2}", amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(format_money(39.99), "$39.99");
        assert_eq!(format_money(3.0), "$3.00");
        assert_eq!(format_money(3.10), "$3.10");
        assert_eq!(format_money(314.16), "$314.16");
    }
}
