// https://www.codewars.com/kata/52f787eb172a8b4ae1000a34/

fn zeros(n: u64) -> u64 {
     // your code here
    let mut result = 0;
    let mut i = 5;
    while n / i >= 1 {
        result += n / i;
        i *= 5;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(17), 3);
        assert_eq!(zeros(24), 4);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }    
}