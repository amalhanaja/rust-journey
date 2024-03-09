// https://www.codewars.com/kata/57a5c31ce298a7e6b7000334/

fn bin_to_decimal(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap()
}

#[test]
fn test_bin_to_decimal() {
    assert_eq!(bin_to_decimal("0"), 0);
    assert_eq!(bin_to_decimal("1"), 1);
    assert_eq!(bin_to_decimal("1001001"), 73);
}
