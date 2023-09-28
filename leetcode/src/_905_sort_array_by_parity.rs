struct Solution;
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut odds: Vec<i32> = vec![];
        let mut evens: Vec<i32> = vec![];
        nums.iter().for_each(|num| {
            if num % 2 == 0 {
                evens.push(*num);
            } else {
                odds.push(*num);
            }
        });
        evens.extend(odds);
        return evens;
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::_905_sort_array_by_parity::Solution;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![2, 4, 3, 1]
        );
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }
}
