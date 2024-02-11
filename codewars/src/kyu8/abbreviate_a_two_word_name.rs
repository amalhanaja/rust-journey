// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/
fn abbrev_name(name: &str) -> String {
    let res: Vec<String> = name
        .split(" ")
        .map(|s| s.chars().next().unwrap().to_string().to_uppercase())
        .collect();
    res.join(".")
}

// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}
