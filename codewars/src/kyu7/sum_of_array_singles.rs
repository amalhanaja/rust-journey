// https://www.codewars.com/kata/59f11118a5e129e591000134/

use std::collections::HashMap;

fn repeats(arr: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    arr.iter().for_each(|x| {
        map.insert(*x, map.get(x).unwrap_or(&0) + 1);
    });
    map.iter().filter(|x| x.1 == &1).map(|x| x.0).sum()
}

#[cfg(test)]
mod tests {
    use super::repeats;

    #[test]
    fn basic_tests() {
        assert_eq!(repeats(&vec![4, 5, 7, 5, 4, 8]), 15);
        assert_eq!(repeats(&vec![9, 10, 19, 13, 19, 13]), 19);
        assert_eq!(repeats(&vec![16, 0, 11, 4, 8, 16, 0, 11]), 12);
        assert_eq!(repeats(&vec![5, 17, 18, 11, 13, 18, 11, 13]), 22);
        assert_eq!(repeats(&vec![5, 10, 19, 13, 10, 13]), 24);
    }
}
