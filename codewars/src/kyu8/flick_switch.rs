// https://www.codewars.com/kata/64fbfe2618692c2018ebbddb/
fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut prev = true;
    list.into_iter()
        .map(|&v| {
            if v == "flick" {
                prev = !prev;
            }
            prev
        })
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::flick_switch;

    fn test_flick<'a, S: Borrow<[&'a str]>, E: Borrow<[bool]>>(strings: S, expected: E) {
        let strings: &[&'a str] = strings.borrow();
        let expected: &[bool] = expected.borrow();
        assert_eq!(flick_switch(strings), expected);
    }

    #[test]
    fn fixed_tests() {
        test_flick(
            ["codewars", "flick", "code", "wars"],
            [true, false, false, false],
        );
        test_flick(
            ["flick", "11037", "3.14", "53"],
            [false, false, false, false],
        );
        test_flick(
            ["false", "false", "flick", "sheep", "flick"],
            [true, true, false, false, true],
        );
        test_flick(["bicycle"], [true]);
        test_flick(["john, smith, susan", "flick"], [true, false]);
        test_flick(
            ["flick", "flick", "flick", "flick", "flick"],
            [false, true, false, true, false],
        );
        test_flick([], []);
    }
}
