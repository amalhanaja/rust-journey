// https://www.codewars.com/kata/55fd2d567d94ac3bc9000064/
fn row_sum_odd_numbers(n: i64) -> i64 {
    let mut odd_numbers: Vec<i64> = Vec::from([1]);
    let mut result: i64 = 1;
    (1..n).for_each(|i| {
        result = 0;
        (0..=i).for_each(|_| {
            let x = odd_numbers.last().unwrap_or(&0) + 2;
            result += x;
            odd_numbers.push(x);
        });
    });
    result
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(2), 8);
    assert_eq!(row_sum_odd_numbers(3), 27);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
