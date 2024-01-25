// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even";
    }
    return "Odd";
}

#[cfg(test)]
mod tests {
    use super::even_or_odd;


    #[test]
    fn test_cases() {
        assert_eq!(even_or_odd(2), "Even");
        assert_eq!(even_or_odd(1), "Odd");
    }

}