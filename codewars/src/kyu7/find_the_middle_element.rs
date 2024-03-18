// https://www.codewars.com/kata/545a4c5a61aa4c6916000755/
fn gimme(input_array: [i32; 3]) -> usize {
    let mut arr = input_array.to_vec();
    arr.sort();
    input_array.iter().position(|&x| x == arr[1]).unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        assert_eq!(gimme([-2, -3, -1]), 0);
        assert_eq!(gimme([5, 10, 14]), 1);
    }
}
