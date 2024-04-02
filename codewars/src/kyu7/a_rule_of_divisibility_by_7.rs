// https://www.codewars.com/kata/55e6f5e58f7817808e00002e
fn seven(n: i64) -> (i64, i32) {
    seven_rec(n, 0)
}

fn seven_rec(n: i64, count: i32) -> (i64, i32) {
    let n_string = n.to_string();
    let len = n_string.len();
    if len > 2 {
        let last = n_string.chars().last().unwrap().to_digit(10).unwrap() as i64;
        return seven_rec(
            n_string[..len - 1].parse::<i64>().unwrap() - (2 * last),
            count + 1,
        );
    }
    return (n, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, exp: (i64, i32)) -> () {
        println!(" n: {:?};", n);
        let ans = seven(n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(477557101, (28, 7));
        dotest(477557102, (47, 7));
        dotest(1603, (7, 2));
    }
}
