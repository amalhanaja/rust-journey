// https://www.codewars.com/kata/56606694ec01347ce800001b

fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    let mut values = vec![a,b,c];
    values.sort();
    return values.pop().unwrap() < values.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::is_triangle;
        
    #[test]
    fn works_for_some_examples() {
        assert_eq!(is_triangle(1, 2, 2), true);
        assert_eq!(is_triangle(7, 2, 2), false);
        assert_eq!(is_triangle(1, 2, 3), false);
        assert_eq!(is_triangle(1, 3, 2), false);
        assert_eq!(is_triangle(3, 1, 2), false);
        assert_eq!(is_triangle(5, 1, 2), false);
        assert_eq!(is_triangle(1, 2, 5), false);
        assert_eq!(is_triangle(2, 5, 1), false);
        assert_eq!(is_triangle(4, 2, 3), true);
        assert_eq!(is_triangle(5, 1, 5), true);
        assert_eq!(is_triangle(2, 2, 2), true);
        assert_eq!(is_triangle(-1, 2, 3), false);
        assert_eq!(is_triangle(1, -2, 3), false);
        assert_eq!(is_triangle(1, 2, -3), false);
        assert_eq!(is_triangle(0, 2, 3), false);
    }
}