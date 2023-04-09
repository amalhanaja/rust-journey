// https://leetcode.com/problems/compare-version-numbers/

struct Solution;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = Solution::split_version(version1);
        let version2 = Solution::split_version(version2);
        for i in 0..(version1.len().max(version2.len())) {
            let v1 = version1.get(i).unwrap_or(&0);
            let v2 = version2.get(i).unwrap_or(&0);
            if v1 < v2 {
                return -1;
            }
            if v1 > v2 {
                return 1;
            }
        }
        return 0;
    }

    fn split_version(version: String) -> Vec<i32> {
        return version
            .split(".")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        // )
    }
}

#[cfg(test)]
mod tests {
    use crate::_165_compare_version::Solution;

    #[test]
    fn sample_test() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_string(), "1.1".to_string()),
            -1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_string(), "1".to_string()),
            1
        );
    }
}
