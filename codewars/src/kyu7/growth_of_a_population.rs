// https://www.codewars.com/kata/563b662a59afc2b5120000c6/train/rust
fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut pop = p0;
    let multiplier : f64 = 1 as f64 + percent/100.0;
    let mut counter = 0;
    while pop < p {
        pop = (pop as f64 * multiplier).floor() as i32 + aug;
        counter += 1;
    }
    
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
    }
}
