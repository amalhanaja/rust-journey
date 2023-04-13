struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut popped_index = 0;
        pushed.iter().for_each(|p|{
            stack.push(p.to_owned());
            while !stack.is_empty() && stack.last() == popped.get(popped_index) {
                stack.pop();
                popped_index += 1;
            }
        });
        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use crate::_946_validate_stack_sequences::Solution;


    #[test]
    fn sample_test() {
        assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]), true);
        assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,3,5,1,2]), false);
    }

}