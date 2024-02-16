// https://www.codewars.com/kata/53af2b8861023f1d88000832/
fn are_you_playing_banjo(name: &str) -> String {
    match name.get(0..1).unwrap() {
        "R" => format!("{} plays banjo", name),
        "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(
            are_you_playing_banjo("Martin"),
            "Martin does not play banjo"
        );
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
