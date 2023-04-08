// https://www.codewars.com/kata/5266876b8f4bf2da9b000362/

fn likes(names: &[&str]) -> String {
    let len = names.to_owned().len();
    match len {
        0 => return String::from("no one likes this"),
        1 => return format!("{} likes this", names[0]),
        2 => return format!("{} and {} like this", names[0], names[1]),
        3 => return format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _default => return format!("{}, {} and {} others like this", names[0], names[1], len - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::likes;

    #[test]
    fn example_tests() {
        assert_eq!(likes(&[]), "no one likes this");
        assert_eq!(likes(&["Peter"]), "Peter likes this");
        assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
        assert_eq!(
            likes(&["Max", "John", "Mark"]),
            "Max, John and Mark like this"
        );
        assert_eq!(
            likes(&["Alex", "Jacob", "Mark", "Max"]),
            "Alex, Jacob and 2 others like this"
        );
    }
}