use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter()
            .for_each(|&num| *map.entry(num).or_insert(0) += 1);
        let mut tuple: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
        tuple.sort_by(|x, y| y.1.cmp(&x.1));
        return tuple.iter().take(k as usize).map(|t| t.0).collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::_347_top_k_frequent_elements::Solution;

    #[test]
    fn test_case() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        )
    }
}
