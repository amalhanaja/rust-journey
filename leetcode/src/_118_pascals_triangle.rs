// https://leetcode.com/problems/pascals-triangle/

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        let mut prev_row: Vec<i32> = Vec::with_capacity(num_rows as usize);
        for i in 0..num_rows as usize {
            let mut current_row = vec![1; i + 1];

            for j in 1..i {
                current_row[j] = prev_row[j - 1] + prev_row[j];
            }

            result.push(current_row.clone());
            prev_row = current_row;
        }
        return result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::generate(5),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::generate(1), [[1]])
    }
}
