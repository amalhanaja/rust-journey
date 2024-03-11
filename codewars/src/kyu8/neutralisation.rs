// https://www.codewars.com/kata/65128732b5aff40032a3d8f0/

fn neutralise(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .map(|(c1, c2)| match c1.cmp(&c2) {
            std::cmp::Ordering::Equal => c1,
            _ => '0',
        })
        .collect()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::neutralise;

    fn dotest(s1: &str, s2: &str, expected: &str) {
        let actual = neutralise(s1, s2);
        assert!(
            actual == expected,
            "With s1 = \"{s1}\" and s2 = \"{s2}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("--++--", "++--++", "000000");
        dotest("-+-+-+", "-+-+-+", "-+-+-+");
        dotest("-++-", "-+-+", "-+00");
        dotest("--++", "++++", "00++");
        dotest("+++--+---", "++----++-", "++0--000-");
        dotest("-----", "-----", "-----");
        dotest("-+", "++", "0+");
        dotest("--", "-+", "-0");
        dotest("-++", "+--", "000");
        dotest("++-++--++-", "-+++-++-++", "0+0+0000+0");
        dotest("-++-+-++-", "+-++++---", "00+0+000-");
        dotest("---++-+--", "-+++--++-", "-00+0-+0-");
        dotest("+-----+++-", "--+-+-++--", "0-0-0-++0-");
        dotest("+-----+-", "---++-++", "0--00-+0");
        dotest("-+--+-+---", "-+--+-+-+-", "-+--+-+-0-");
        dotest("+-+", "-++", "00+");
        dotest("-++", "-+-", "-+0");
        dotest("---+", "-+++", "-00+");
        dotest("+--", "+--", "+--");
        dotest("--+++-+-", "+++++---", "00+++-0-");
    }
}
