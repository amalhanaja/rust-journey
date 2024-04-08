fn disarium_number(n: u32) -> String {
    if n.to_string()
        .char_indices()
        .map(|(i, c)| c.to_digit(10).unwrap().pow(i as u32 + 1))
        .sum::<u32>()
        == n
    {
        return "Disarium !!".to_string();
    }
    "Not !!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(disarium_number(89), "Disarium !!");
        assert_eq!(disarium_number(564), "Not !!");
        assert_eq!(disarium_number(1024), "Not !!");
        assert_eq!(disarium_number(135), "Disarium !!");
        assert_eq!(disarium_number(136586), "Not !!");
    }
}
