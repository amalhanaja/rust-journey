// https://www.codewars.com/kata/57f222ce69e09c3630000212/
fn well(x: &[&str]) -> &'static str {
    match x.into_iter().filter(|&&s| s == "good").count() {
        0 => "Fail!",
        1 | 2 => "Publish!",
        _ => "I smell a series!",
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(well(&["bad", "bad", "bad"]), "Fail!");
        assert_eq!(well(&["good", "bad", "bad", "bad"]), "Publish!");
        assert_eq!(
            well(&["good", "bad", "bad", "bad", "bad", "good", "bad", "bad", "good"]),
            "I smell a series!"
        );
    }
}
