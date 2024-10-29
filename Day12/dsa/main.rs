impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
         let mut nums = nums;
    nums.sort();  // Sort the array
    let mut result = Vec::new();
    
    for i in 0..nums.len() {
        // Skip duplicate elements to ensure unique triplets
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        
        let (mut left, mut right) = (i + 1, nums.len() - 1);
        
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            
            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                
                // Move pointers to avoid duplicates
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    result
    }
}

struct Solution;

fn main() {
    let nums = vec![1,2,3,4,5,6];
    let result = Solution::three_sum(nums);
    println!("{:?}", result); // Expected output: [[-1, -1, 2], [-1, 0, 1]]
}
