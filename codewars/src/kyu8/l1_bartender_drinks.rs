use std::collections::HashMap;

// https://www.codewars.com/kata/568dc014440f03b13900001d/
fn get_drink_by_profession(param: &str) -> &'static str {
    // code me!
    let table = HashMap::from([
        ("jabroni", "Patron Tequila"),
        ("school counselor", "Anything with Alcohol"),
        ("programmer", "Hipster Craft Beer"),
        ("bike gang member", "Moonshine"),
        ("politician", "Your tax dollars"),
        ("rapper", "Cristal"),
    ]);
    table.get(&*param.to_lowercase()).unwrap_or(&"Beer")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(
            get_drink_by_profession("jabrOni"),
            "Patron Tequila",
            "'Jabroni' should map to 'Patron Tequila'"
        );
        assert_eq!(
            get_drink_by_profession("scHOOl counselor"),
            "Anything with Alcohol",
            "'School Counselor' should map to 'Anything with alcohol'"
        );
        assert_eq!(
            get_drink_by_profession("prOgramMer"),
            "Hipster Craft Beer",
            "'Programmer' should map to 'Hipster Craft Beer'"
        );
        assert_eq!(
            get_drink_by_profession("bike ganG member"),
            "Moonshine",
            "'Bike Gang Member' should map to 'Moonshine'"
        );
        assert_eq!(
            get_drink_by_profession("poLiTiCian"),
            "Your tax dollars",
            "'Politician' should map to 'Your tax dollars'"
        );
        assert_eq!(
            get_drink_by_profession("rapper"),
            "Cristal",
            "'Rapper' should map to 'Cristal'"
        );
        assert_eq!(
            get_drink_by_profession("pundit"),
            "Beer",
            "'Pundit' should map to 'Beer'"
        );
        assert_eq!(
            get_drink_by_profession("Pug"),
            "Beer",
            "'Pug' should map to 'Beer'"
        );
    }
}
