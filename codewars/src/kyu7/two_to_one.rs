use std::collections::HashSet;

// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/
fn longest(a1: &str, a2: &str) -> String {
    let set = add_to_set(HashSet::new(), a1);
    let set = add_to_set(set, a2);
    let mut vec: Vec<char> = set.into_iter().collect();
    vec.sort();
    vec.iter().collect()
}

fn add_to_set(mut set: HashSet<char>, s: &str) -> HashSet<char> {
    s.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
