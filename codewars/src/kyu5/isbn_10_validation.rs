// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001

fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10 {
        return false;
    }
    let isbn_chars = isbn
        .char_indices()
        .filter(|(i, c)| (c.is_digit(10)) || (i == &9 && c == &'X'));
    if isbn_chars.clone().count() != 10 {
        return false;
    }
    let sum = isbn_chars.fold(0, |mut acc: u64, (i, c)| {
        let normalized_index: u64 = i as u64 + 1;
        let digit = if c == 'X' {
            10
        } else {
            c.to_digit(10).unwrap()
        } as u64;
        acc += digit * normalized_index;
        acc
    });
    if sum == 0 {
        return false;
    }
    return sum % 11 == 0;
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(
            actual == expected,
            "Test failed with isbn = {isbn}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("X123456788", false);
        dotest("ABCDEFGHIJ", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
        dotest("3X97343520", false);
    }
}
