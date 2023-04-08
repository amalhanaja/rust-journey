// https://www.codewars.com/kata/550498447451fbbd7600041c

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut set_a: Vec<i64> = a.iter().map(|x| x.pow(2)).collect();
    let mut set_b: Vec<i64> = b.iter().map(|x| x.to_owned()).collect();
    set_a.sort();
    set_b.sort();
    return set_a == set_b;
}

fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
    assert_eq!(comp(a, b), exp)
}

#[test]
fn tests_comp() {
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 11,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    testing(a1, a2, false);
}
