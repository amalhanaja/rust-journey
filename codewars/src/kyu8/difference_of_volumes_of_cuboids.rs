// https://www.codewars.com/kata/58cb43f4256836ed95000f97/
fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    volume(a).abs_diff(volume(b)) as i32
}

fn volume(cube: &[i32; 3]) -> i32 {
    cube.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_difference(&[3, 2, 5], &[1, 4, 4]), 14);
        assert_eq!(find_difference(&[9, 7, 2], &[5, 2, 2]), 106);
        assert_eq!(find_difference(&[11, 2, 5], &[1, 10, 8]), 30);
        assert_eq!(find_difference(&[4, 4, 7], &[3, 9, 3]), 31);
        assert_eq!(find_difference(&[15, 20, 25], &[10, 30, 25]), 0);
    }
}
