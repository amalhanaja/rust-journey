// https://www.codewars.com/kata/5a58d889880385c2f40000aa/

fn automorphic(n: u64) -> String {
    let last_squared = n
        .pow(2)
        .to_string()
        .chars()
        .rev()
        .take(n.to_string().len())
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    ["Not!!", "Automorphic"][(last_squared == n) as usize].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(automorphic(1), "Automorphic");
        assert_eq!(automorphic(3), "Not!!");
        assert_eq!(automorphic(6), "Automorphic");
        assert_eq!(automorphic(9), "Not!!");
        assert_eq!(automorphic(25), "Automorphic");
        assert_eq!(automorphic(53), "Not!!");
        assert_eq!(automorphic(76), "Automorphic");
        assert_eq!(automorphic(95), "Not!!");
        assert_eq!(automorphic(625), "Automorphic");
        assert_eq!(automorphic(225), "Not!!");
    }
}
