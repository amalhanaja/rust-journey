// https://www.codewars.com/kata/566fc12495810954b1000030/
fn nb_dig(n: i32, d: i32) -> i32 {
    (0..=n)
        .map(|x| {
            x.pow(2)
                .to_string()
                .chars()
                .filter(|c| c.to_digit(10).unwrap() as i32 == d)
                .count() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10, 1, 4);
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
