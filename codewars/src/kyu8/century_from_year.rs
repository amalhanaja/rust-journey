// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/
fn century(year: u32) -> u32 {
    if year % 100 == 0 {
        return year / 100
    }
    year / 100 + 1
}

#[test]
fn basic_tests() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
}
