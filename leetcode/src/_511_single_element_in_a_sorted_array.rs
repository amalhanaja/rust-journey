// https://leetcode.com/problems/single-element-in-a-sorted-array/

struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let left_index = 0;
        let right_index = nums.len() - 1;
        return Solution::single_non_duplicate_recursive(nums, left_index, right_index);
    }

    fn single_non_duplicate_recursive(
        nums: Vec<i32>,
        left_index: usize,
        right_index: usize,
    ) -> i32 {
        if left_index >= right_index {
            return nums[left_index];
        }
        let mid = (left_index + right_index) / 2;
        if mid % 2 == 0 {
            if nums[mid] == nums[mid + 1] {
                return Solution::single_non_duplicate_recursive(nums, mid + 2, right_index);
            } else {
                return Solution::single_non_duplicate_recursive(nums, left_index, mid);
            }
        } else {
            if nums[mid] == nums[mid - 1] {
                return Solution::single_non_duplicate_recursive(nums, mid + 1, right_index);
            } else {
                return Solution::single_non_duplicate_recursive(nums, left_index, mid - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        // When
        let result = Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]);

        // Then
        assert_eq!(2, result)
    }

    #[test]
    fn test_case_2() {
        // When
        let result = Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]);

        // Then
        assert_eq!(10, result)
    }
}
