// https://www.codewars.com/kata/59cfc000aeb2844d16000075/
fn capitalize(s: &str) -> Vec<String> {
    s.char_indices()
        .map(|(i, c)| (i, [c.to_ascii_lowercase(), c.to_ascii_uppercase()]))
        .fold(
            Vec::from(["".to_string(), "".to_string()]),
            |acc: Vec<String>, (i, c)| {
                vec![
                    format!("{}{}", acc[0], c[(i % 2 == 0) as usize]),
                    format!("{}{}", acc[1], c[(i % 2 == 1) as usize]),
                ]
            },
        )
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(capitalize("abcdef"), ["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"), ["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"), ["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"), ["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
    }
}
