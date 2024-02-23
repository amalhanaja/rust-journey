// https://www.codewars.com/kata/57a429e253ba3381850000fb/

fn bmi(weight: u32, height: f32) -> &'static str {
    match (weight as f32) / height.powi(2) {
        bmi if bmi <= 18.5 => "Underweight",
        bmi if bmi <= 25.0 => "Normal",
        bmi if bmi <= 30.0 => "Overweight",
        _  => "Obese"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}
