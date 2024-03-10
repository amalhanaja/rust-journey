// https://www.codewars.com/kata/57e92e91b63b6cbac20001e5/
fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    ((holiday_cost as f32) / (price as f32) / (discount as f32) * 100.) as i32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(duty_free(10, 10, 500), 500);
        assert_eq!(duty_free(12, 50, 1000), 166);
        assert_eq!(duty_free(17, 10, 500), 294);
    }
}
