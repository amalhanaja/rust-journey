struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result: String = String::new();
        let mut star_count = 0;
        s.chars().rev().for_each(| c| {
            match c {
                '*' => {
                    star_count += 1;
                },
                _ => {
                    if star_count > 0 {
                        star_count -= 1;
                    } else {
                        result.insert(0, c);
                    }
                }
            }
        });
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::_2390_removing_stars_from_a_string::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
        assert_eq!(Solution::remove_stars("erase*****".to_string()), "".to_string());
    }
}
