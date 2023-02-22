// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/

struct Solution;
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let left = weights.clone().iter().max().unwrap().to_owned();
        let right = weights.clone().iter().fold(0, |acc, current| acc + current);
        return Solution::ship_with_days_recurive(weights, days, left, right);
    }

    fn ship_with_days_recurive(weights: Vec<i32>, days: i32, left: i32, right: i32) -> i32 {
        if left >= right {
            return left
        }
        let mid = (left + right) / 2;
        let mut load = 0;
        let mut count = 1;
        for weight in weights.clone().iter() {
            load += weight;
            if load > mid {
                count += 1;
                load = *weight;
            }
        }
        if count <= days {
            return Solution::ship_with_days_recurive(weights, days, left, mid)
        } else {
            return Solution::ship_with_days_recurive(weights, days, mid+1, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        // When
        let result = Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);

        // Then
        assert_eq!(15, result);
    }

    #[test]
    fn test_case_2() {
        // When
        let result = Solution::ship_within_days(vec![3,2,2,4,1,4], 3);

        // Then
        assert_eq!(6, result);
    }

    #[test]
    fn test_case_3() {
        // When
        let result = Solution::ship_within_days(vec![1,2,3,1,1], 4);

        // Then
        assert_eq!(3, result);
    }
}
