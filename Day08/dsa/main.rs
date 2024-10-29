struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut combination = Vec::new();
        Solution::backtrack(&candidates, target, 0, &mut combination, &mut results);
        results
    }
    
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        combination: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>
    ) {
        if target == 0 {
            results.push(combination.clone());
            return;
        }
        
        for i in start..candidates.len() {
            if candidates[i] > target {
                continue;
            }
            combination.push(candidates[i]);
            Solution::backtrack(candidates, target - candidates[i], i, combination, results);
            combination.pop();
        }
    }
}

fn main() {
    let candidates = vec![2, 3, 6, 7];
    let target = 10;
    let result = Solution::combination_sum(candidates, target);
    println!("{:?}", result); // Expected output: [[2, 2, 3], [7]]
}
