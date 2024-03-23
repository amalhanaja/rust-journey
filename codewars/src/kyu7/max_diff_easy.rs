// https://www.codewars.com/kata/588a3c3ef0fbc9c8e1000095/

fn max_diff(numbers: &[i32]) -> i32 {
    let min = numbers.iter().min().unwrap_or(&0);
    let max = numbers.iter().max().unwrap_or(&0);
    min.abs_diff(*max) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 6]), 6);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(max_diff(&[-0, 1, 2, -3, 4, 5, -6]), 11);
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(max_diff(&[0, 1, 2, 3, 4, 5, 16]), 16);
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(max_diff(&[16]), 0);
    }

    #[test]
    fn test_sample_5() {
        assert_eq!(max_diff(&[]), 0);
    }
}
