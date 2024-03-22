// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/
fn row_weights(array: Vec<u32>) -> (u32, u32) {
    let team_1: u32 = array.iter().step_by(2).sum();
    let team_2: u32 = array.iter().skip(1).step_by(2).sum();
    (team_1, team_2)
}

#[test]
fn basic_tests() {
    assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
    assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
    assert_eq!(row_weights(vec![80]), (80, 0));
}
