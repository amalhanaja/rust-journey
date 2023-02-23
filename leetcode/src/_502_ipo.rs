// https://leetcode.com/problems/ipo/

use std::collections::BinaryHeap;

struct Solution;

struct Project {
    capital: i32,
    profit: i32,
}
impl Project {
    pub fn new(capital: i32, profit: i32) -> Self {
        return Self { capital, profit };
    }
}

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        // Hold capital and profit in pair
        let mut projects: Vec<Project> = profits
            .iter()
            .zip(capital.iter())
            .map(|(&p, &c)| Project::new(c, p))
            .collect();

        // Sort project from minimum captial
        projects.sort_by_key(|p| p.capital);

        let mut priority_queue: BinaryHeap<i32> = BinaryHeap::new();
        let mut index = 0;
        let mut current_capital = w;

        // Loop from 0 to k
        for _ in 0..k {
            // Add all profit to priority_queue having capital <= current_capital
            while index < projects.len() && projects[index].capital <= current_capital {
                priority_queue.push(projects[index].profit);
                index += 1;   
            } 
            
            // Pop fro priority queue then add to current_capital
            if let Some(p) = priority_queue.pop() {
                current_capital += p
            } else {
                break;
            }
        }

        return current_capital;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        // When
        let result = Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]);

        // Then
        assert_eq!(4, result)
    }

    #[test]
    fn test_case_2() {
        // When
        let result = Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]);

        // Then
        assert_eq!(6, result)
    }
}
