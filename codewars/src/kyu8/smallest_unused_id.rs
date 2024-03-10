// https://www.codewars.com/kata/55eea63119278d571d00006a/
fn next_id(ids: &[usize]) -> usize {
    //this will be awesome!
    let mut ids = Vec::from(ids);
    ids.sort();
    ids.dedup();
    for (i, &v) in ids.iter().enumerate() {
        if i != v {
            return i;
        }
    }
    ids.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(next_id(&[0, 1, 2, 4, 5]), 3);
        assert_eq!(next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
    }
}
