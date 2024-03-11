// https://www.codewars.com/kata/55dc4520094bbaf50e0000cb/
fn am_i_wilson(n: u32) -> bool {
    [5, 13, 563].contains(&n)
}

#[test]
fn should_return_false_for_composite_numbers() {
    assert_eq!(am_i_wilson(9), false);
    assert_eq!(am_i_wilson(6), false);
}

#[test]
fn should_return_false_for_nonwilson_primes() {
    assert_eq!(am_i_wilson(2), false);
    assert_eq!(am_i_wilson(17), false);
}

#[test]
fn should_return_true_for_wilson_primes() {
    assert_eq!(am_i_wilson(5), true);
}

#[test]
fn should_be_able_to_check_bigger_numbers() {
    assert_eq!(am_i_wilson(307), false);
}

#[test]
fn should_return_false_for_wilson_composites() {
    assert_eq!(am_i_wilson(5971), false);
}
