// https://www.codewars.com/kata/52fba66badcd10859f00097e/
fn disemvowel(s: &str) -> String {
    s.chars().filter(|&c| !"aiueoAIUEO".contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
