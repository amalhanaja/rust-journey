// https://www.codewars.com/kata/5412509bd436bd33920011bc/

/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    if cc.len() <= 4 {
        return cc.to_string();
    }
    "#".repeat(cc.len() - 4) + &cc[cc.len() - 4..]
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify("11111"), "#1111");
    }
}
