// https://www.codewars.com/kata/58712dfa5c538b6fc7000569/

fn count_red_beads(n: u32) -> u32 {
    (0..n)
        .map(|_| format!("@"))
        .collect::<Vec<_>>()
        .join("++")
        .chars()
        .filter(|&x| x != '@')
        .count() as u32
}

#[test]
fn test() {
    assert_eq!(count_red_beads(0), 0);
    assert_eq!(count_red_beads(1), 0);
    assert_eq!(count_red_beads(3), 4);
    assert_eq!(count_red_beads(5), 8);
}
