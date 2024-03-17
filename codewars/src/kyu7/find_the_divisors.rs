// https://www.codewars.com/kata/544aed4c4a30184e960010f4/

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut divisor: Vec<u32> = Vec::new();
    (2..=integer / 2).for_each(|x| {
        if integer % x == 0 {
            divisor.push(x);
        }
    });
    if divisor.is_empty() {
        return Err(format!("{} is prime", integer));
    }
    Ok(divisor)
}

#[test]
fn tests() {
    assert_eq!(divisors(15), Ok(vec![3, 5]));
    assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
    assert_eq!(divisors(13), Err("13 is prime".to_string()));
}
