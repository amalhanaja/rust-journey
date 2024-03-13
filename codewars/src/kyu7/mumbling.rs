// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/
fn accum(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| {
            format!(
                "{}{}",
                c.to_ascii_uppercase(),
                repeat(i, c.to_ascii_lowercase())
            )
        })
        .collect::<Vec<_>>()
        .join("-")
}

fn repeat(count: usize, c: char) -> String {
    (0..count).map(|_| format!("{}", c)).collect()
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn basic_tests() {
    assert_eq!(
        accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("NyffsGeyylB"),
        "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
    );
    assert_eq!(
        accum("MjtkuBovqrU"),
        "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
    );
    assert_eq!(
        accum("EvidjUnokmM"),
        "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
    );
    assert_eq!(
        accum("HbideVbxncC"),
        "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
    );
}
