use std::collections::HashMap;

// https://www.codewars.com/kata/57a6633153ba33189e000074/
fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut map: HashMap<char, i32> = HashMap::new();
    sip.chars().for_each(|c| {
        map.insert(c, map.get(&c).unwrap_or(&0) + 1);
    });
    let mut vec: Vec<_> = map.into_iter().collect();
    vec.sort_by(|a, b| {
        sip.chars()
            .position(|c| c == a.0)
            .unwrap()
            .cmp(&sip.chars().position(|c| c == b.0).unwrap())
    });
    vec
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
