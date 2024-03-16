// https://www.codewars.com/kata/578553c3a1b8d5c40300037c/
fn binary_slice_to_number(slice: &[u32]) -> u32 {
    let s: String = slice.iter().map(|x| x.to_string()).collect();
    u32::from_str_radix(&*s, 2).unwrap()
}
#[cfg(test)]
mod tests {
    use super::binary_slice_to_number;

    #[test]
    fn example_tests() {
        assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
        assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
        assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
        assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
    }
}
