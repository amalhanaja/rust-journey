// https://www.codewars.com/kata/5208f99aee097e6552000148

fn solution(s: &str) -> String {
    // let mut result = "";
    let result = s.chars().fold(String::new(), |mut acc, c| {
        if c.is_uppercase() {
            acc.push(' ');
        }
        acc.push(c);
        return acc;
    });
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
