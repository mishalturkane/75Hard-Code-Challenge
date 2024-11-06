use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new(); // Initialize the hash map

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num; // Calculate the needed complement

            // Check if the complement is in the map
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32]; // Return indices if complement is found
            }

            // Store the number and its index in the map
            map.insert(num, i);
        }

        vec![] // Return an empty vector if no solution is found
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6,10];
    let target = 14;
    let result = Solution::two_sum(arr, target);
    println!("{:?}", result); // Expected output: [1, 3]
}
