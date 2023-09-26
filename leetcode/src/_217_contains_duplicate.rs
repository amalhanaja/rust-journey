use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut sets: HashSet<&i32> = HashSet::new();
        nums.iter().for_each(|num| {
            sets.insert(num);
        });
        return sets.len() != nums.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::_217_contains_duplicate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
