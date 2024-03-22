// https://www.codewars.com/kata/5663f5305102699bad000056/

fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    match (
        a1.clone()
            .into_iter()
            .max_by(|&a, &b| a.len().cmp(&b.len())),
        a1.into_iter().min_by(|&a, &b| a.len().cmp(&b.len())),
        a2.clone()
            .into_iter()
            .max_by(|&a, &b| a.len().cmp(&b.len())),
        a2.into_iter().min_by(|&a, &b| a.len().cmp(&b.len())),
    ) {
        (Some(a1_max), Some(a1_min), Some(a2_max), Some(a2_min)) => a1_min
            .len()
            .abs_diff(a2_max.len())
            .max(a2_min.len().abs_diff(a1_max.len()))
            as i32,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
    }
}
