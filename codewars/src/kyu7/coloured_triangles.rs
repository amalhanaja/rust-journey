// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/
fn triangle(row_str: &str) -> String {
    let chars = row_str.chars().collect::<Vec<_>>();
    triangle_recursive(chars)
}

fn triangle_recursive(chars: Vec<char>) -> String {
    if chars.len() == 1 {
        return format!("{}", chars[0]);
    }
    let reduced = chars
        .windows(2)
        .map(|x| match (x[0], x[1]) {
            ('G', 'R') | ('R', 'G') => 'B',
            ('B', 'G') | ('G', 'B') => 'R',
            ('B', 'R') | ('R', 'B') => 'G',
            _ => x[0],
        })
        .collect::<Vec<char>>();
    triangle_recursive(reduced)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        // assert_eq!(triangle("GB"), "R");
        // assert_eq!(triangle("RRR"), "R");
        // assert_eq!(triangle("RGBG"), "B");
        // assert_eq!(triangle("RBRGBRB"), "G");
        // assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
        // assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("B"), "B");
    }
}
