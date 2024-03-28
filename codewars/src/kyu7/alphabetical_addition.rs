// https://www.codewars.com/kata/5d50e3914861a500121e1958/

fn add_letters(letters: Vec<char>) -> char {
    let out = letters.iter().fold(0, |acc, &x| acc + x as i32 - 96) % 26;
    if out == 0 {
        return 'z';
    }
    (out + 96) as u8 as char
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
}
