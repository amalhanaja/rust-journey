// https://www.codewars.com/kata/5a651865fd56cb55760000e0/
fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let mut sum_to_right = arr.iter().sum::<i32>();
    let mut result = Vec::<i32>::new();
    arr.iter().for_each(|&x| {
        sum_to_right -= x;
        if x > sum_to_right {
            result.push(x);
        }
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1, 2, 3, 4, 0]), [4]);
        assert_eq!(array_leaders(&[16, 17, 4, 3, 5, 2]), [17, 5, 2]);

        // Negative values
        assert_eq!(array_leaders(&[-1, -29, -26, -2]), [-1]);
        assert_eq!(array_leaders(&[-36, -12, -27]), [-36, -12]);

        // Mixed values
        assert_eq!(array_leaders(&[5, -2, 2]), [5, 2]);
        assert_eq!(array_leaders(&[0, -1, -29, 3, 2]), [0, -1, 3, 2]);
    }
}
