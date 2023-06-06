struct Solution;
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let diffs: Vec<_> = arr.windows(2)
            .map(|item| item[1] - item[0])
            .collect();
        if let Some(x) = diffs.first() {
            return diffs.iter().all(|y| x == y)
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::can_make_arithmetic_progression(vec![3,5,1]), true);
        assert_eq!(Solution::can_make_arithmetic_progression(vec![1,2,4]), false);
    }
}
