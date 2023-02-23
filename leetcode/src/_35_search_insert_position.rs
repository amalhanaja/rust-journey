// https://leetcode.com/problems/search-insert-position/

struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let left_index = 0;
        let right_index = nums.len() - 1;
        return Solution::search_insert_recursive(nums, target, left_index, right_index);
    }

    fn search_insert_recursive(nums: Vec<i32>, target: i32, left_index: usize, right_index: usize) -> i32 {
        if left_index > right_index {
            return left_index as i32;
        }
        let mid = left_index + (right_index - left_index) / 2;
        if target == nums[mid]{
            return mid as i32;
        }
        if target < nums[mid] && mid == left_index {
            return mid as i32;
        }
        if target > nums[mid] {
            return Solution::search_insert_recursive(nums, target, mid + 1, right_index)
        }
        return Solution::search_insert_recursive(nums, target, left_index, mid - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;


    #[test]
    fn test_case_1() {
        // When
        let result = Solution::search_insert(vec![1,3,5,6], 5);

        // Then
        assert_eq!(2, result);
    }

    #[test]
    fn test_case_2() {
        // When
        let result = Solution::search_insert(vec![1,3,5,6], 2);

        // Then
        assert_eq!(1, result);
    }

    #[test]
    fn test_case_3() {
        // When
        let result = Solution::search_insert(vec![1,3,5,6], 7);

        // Then
        assert_eq!(4, result);
    }

    #[test]
    fn test_case_4() {
        // When
        let result = Solution::search_insert(vec![1,3,5,6], 0);

        // Then
        assert_eq!(0, result);
    }

    #[test]
    fn test_case_5() {
        // When
        let result = Solution::search_insert(vec![2,5], 1);

        // Then
        assert_eq!(0, result);
    }

}