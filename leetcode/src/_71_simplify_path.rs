struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        if path.is_empty() {
            return path;
        }
        let mut queue: Vec<&str> = Vec::new();
        path.split('/')
            .filter(|s| !(s.is_empty() || s.to_owned() == "."))
            .for_each(|s| {
                if s == ".." {
                    queue.pop();
                } else {
                    queue.push(s);
                }
            });
        return format!("/{}", queue.join("/"));
    }
}

#[cfg(test)]
mod tests {
    use crate::_71_simplify_path::Solution;

    #[test]
    fn sample_test() {
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/a/c".to_string()
        );
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
        assert_eq!(
            Solution::simplify_path("/home//foo".to_string()),
            "/home/foo".to_string()
        );
    }
}
