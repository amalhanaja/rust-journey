// https://www.codewars.com/kata/55685cd7ad70877c23000102


fn make_negative(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    return (n.abs() * -1) as i32;
}

#[cfg(test)]
mod tests {
    use super::make_negative;
    
    #[test]
    fn sample_tests() {
        assert_eq!(make_negative(1), -1);
        assert_eq!(make_negative(-5), -5);
        assert_eq!(make_negative(0), 0);
    }
}