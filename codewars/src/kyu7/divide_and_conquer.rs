// https://www.codewars.com/kata/57eaec5608fed543d6000021/
use either::Either;

fn div_con(arr: &[Either<i32, String>]) -> i32 {
    arr.iter().fold(0i32, |acc, current| match current {
        Either::Left(left) => acc + left,
        Either::Right(right) => acc - right.parse::<i32>().unwrap(),
    })
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::div_con;
    use either::Either;

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = div_con(arr);
        assert!(
            actual == expected,
            "With arr = {arr:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                either::Left(9),
                either::Left(3),
                either::Right("7".to_string()),
                either::Right("3".to_string()),
            ],
            2,
        );
        dotest(
            &[
                Either::Right("5".to_string()),
                Either::Right("0".to_string().to_string()),
                Either::Left(9),
                Either::Left(3),
                Either::Left(2),
                Either::Left(1),
                Either::Right("9".to_string()),
                Either::Left(6),
                Either::Left(7),
            ],
            14,
        );
        dotest(
            &[
                Either::Right("3".to_string()),
                Either::Left(6),
                Either::Left(6),
                Either::Left(0),
                Either::Right("5".to_string()),
                Either::Left(8),
                Either::Left(5),
                Either::Right("6".to_string()),
                Either::Left(2),
                Either::Right("0".to_string()),
            ],
            13,
        );
        dotest(
            &[
                Either::Right("1".to_string()),
                Either::Right("5".to_string()),
                Either::Right("8".to_string()),
                Either::Left(8),
                Either::Left(9),
                Either::Left(9),
                Either::Left(2),
                Either::Right("3".to_string()),
            ],
            11,
        );
        dotest(
            &[
                Either::Left(8),
                Either::Left(0),
                Either::Left(0),
                Either::Left(8),
                Either::Left(5),
                Either::Left(7),
                Either::Left(2),
                Either::Left(3),
                Either::Left(7),
                Either::Left(8),
                Either::Left(6),
                Either::Left(7),
            ],
            61,
        );
    }
}
