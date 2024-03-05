// https://www.codewars.com/kata/5772da22b89313a4d50012f7/
fn greet(name: &str, owner: &str) -> String {
    match name.cmp(owner) {
        std::cmp::Ordering::Equal => "Hello boss",
        _ => "Hello guest",
    }
    .to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(greet("Greg", "Daniel"), "Hello guest");
    }
}
