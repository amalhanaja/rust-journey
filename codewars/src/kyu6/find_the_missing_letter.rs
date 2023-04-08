// https://www.codewars.com/kata/5839edaa6754d6fec10000a2

fn find_missing_letter(chars: &[char]) -> char {
    let mut character: Option<char> = None;
    for c in chars {
        let owned_char = c.to_owned();
        if let Some(last_char) = character {
            let next = last_char as u8 + 1;
            if next != owned_char as u8 {
                return next as char;
            }
        }
        character = Some(owned_char)
    }
    return character.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}
