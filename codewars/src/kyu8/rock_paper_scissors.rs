// https://www.codewars.com/kata/5672a98bdbdd995fad00000f/
fn rps(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("scissors", "paper") => "Player 1 won!",
        ("scissors", "rock") => "Player 2 won!",
        ("paper", "rock") => "Player 1 won!",
        ("paper", "scissors") => "Player 2 won!",
        ("rock", "scissors") => "Player 1 won!",
        ("rock", "paper") => "Player 2 won!",
        _ => "Draw!",
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::rps;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            rps(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
