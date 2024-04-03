use std::collections::HashSet;

// https://www.codewars.com/kata/5a092d9e46d843b9db000064/
fn solve(arr: &Vec<i32>) -> i32 {
    arr.into_iter()
        .cloned()
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_tests() {
        assert_eq!(solve(&vec![1, -1, 2, -2, 3]), 3);
        assert_eq!(solve(&vec![-3, 1, 2, 3, -1, -4, -2]), -4);
        assert_eq!(solve(&vec![1, -1, 2, -2, 3, 3]), 3);
        assert_eq!(
            solve(&vec![-110, 110, -38, -38, -62, 62, -38, -38, -38]),
            -38
        );
        assert_eq!(solve(&vec![-9, -105, -9, -9, -9, -9, 105]), -9);
    }
}
