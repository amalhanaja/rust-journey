// https://www.codewars.com/kata/56541980fa08ab47a0000040/

fn printer_error(s: &str) -> String {
    let error_count = s.chars().filter(|&c| c > 'm').count();
    format!("{}/{}", error_count, s.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass_all_the_tests_provided() {
        assert_eq!(
            &printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "3/56"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"),
            "6/60"
        );
        assert_eq!(
            &printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu"),
            "11/65"
        );
    }
}
