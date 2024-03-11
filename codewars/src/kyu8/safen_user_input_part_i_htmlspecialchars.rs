// https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/

fn html_special_chars(html: &str) -> String {
    let dictionary = [
        ("&", "&amp;"),
        ("<", "&lt;"),
        (">", "&gt;"),
        ("\"", "&quot;"),
    ];
    let mut result = html.to_string();
    for (key, value) in dictionary.into_iter() {
        result = result.replace(key, value)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::html_special_chars;

    #[test]
    fn sample_tests() {
        assert_eq!(
            html_special_chars("<h2>Hello World</h2>"),
            "&lt;h2&gt;Hello World&lt;/h2&gt;"
        );
        assert_eq!(
            html_special_chars("Hello, how would you & I fare?"),
            "Hello, how would you &amp; I fare?"
        );
        assert_eq!(
            html_special_chars("How was \"The Matrix\"?  Did you like it?"),
            "How was &quot;The Matrix&quot;?  Did you like it?"
        );
        assert_eq!(
            html_special_chars("<script>alert('Website Hacked!');</script>"),
            "&lt;script&gt;alert('Website Hacked!');&lt;/script&gt;"
        );
    }
}
