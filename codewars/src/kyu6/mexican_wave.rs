fn wave(s: &str) -> Vec<String> {
    let mut i = 0;
    return s.chars().fold(Vec::new(), |mut acc, c| {
        let mut string = s.to_string();
        if c.is_whitespace() {
            i += 1;
            return acc;
        }
        string.remove(i);
        string.insert(i, c.to_ascii_uppercase());
        acc.push(string);
        i += 1;
        acc
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);

        let expect = [
            "Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs",
            "codewarS",
        ];
        assert_eq!(wave("codewars"), expect);

        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);

        let expect = [
            "Two words",
            "tWo words",
            "twO words",
            "two Words",
            "two wOrds",
            "two woRds",
            "two worDs",
            "two wordS",
        ];
        assert_eq!(wave("two words"), expect);

        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
