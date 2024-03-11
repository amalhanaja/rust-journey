// https://www.codewars.com/kata/5862f663b4e9d6f12b00003b/
fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    let remaining = (blue_start + red_start - blue_pulled - red_pulled) as f32;
    (blue_start - blue_pulled) as f32 / remaining
}

#[test]
fn basic_tests() {
    assert_eq!(guess_blue(5, 5, 2, 3), 0.6);
    assert_eq!(guess_blue(5, 7, 4, 3), 0.2);
    assert_eq!(guess_blue(12, 18, 4, 6), 0.4);
}
