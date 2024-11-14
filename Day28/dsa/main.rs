// arraydsaquestions.rs

use std::collections::{HashMap, HashSet};

// 1. Maximum Product Subarray
fn max_product_subarray(nums: &[i32]) -> i32 {
    let (mut max_prod, mut min_prod, mut result) = (nums[0], nums[0], nums[0]);
    for &num in nums.iter().skip(1) {
        if num < 0 {
            std::mem::swap(&mut max_prod, &mut min_prod);
        }
        max_prod = max_prod.max(num * max_prod);
        min_prod = min_prod.min(num * min_prod);
        result = result.max(max_prod);
    }
    result
}

// 2. Sliding Window Maximum
fn sliding_window_maximum(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = vec![];
    let mut deque: Vec<usize> = vec![];
    
    for i in 0..nums.len() {
        if !deque.is_empty() && deque[0] < i + 1 - k {
            deque.remove(0);
        }
        
        while !deque.is_empty() && nums[*deque.last().unwrap()] < nums[i] {
            deque.pop();
        }
        
        deque.push(i);
        
        if i >= k - 1 {
            result.push(nums[deque[0]]);
        }
    }
    result
}

// 3. Longest Subarray with Sum at Most K
fn longest_subarray_with_sum_at_most_k(nums: &[i32], k: i32) -> i32 {
    let (mut max_length, mut current_sum, mut start) = (0, 0, 0);
    
    for end in 0..nums.len() {
        current_sum += nums[end];
        
        while current_sum > k && start <= end {
            current_sum -= nums[start];
            start += 1;
        }
        
        max_length = max_length.max((end - start + 1) as i32);
    }
    max_length
}

// 4. Count of Smaller Numbers After Self
fn count_smaller_after_self(nums: Vec<i32>) -> Vec<i32> {
    let mut counts = vec![0; nums.len()];
    let mut sorted_nums = vec![];

    for (i, &num) in nums.iter().enumerate().rev() {
        let position = sorted_nums.binary_search(&num).unwrap_or_else(|x| x);
        counts[i] = position as i32;
        sorted_nums.insert(position, num);
    }
    counts
}

// 5. Subarray Sum Equals Zero
fn subarray_sum_equals_zero(nums: &[i32]) -> bool {
    let mut set = HashSet::new();
    let mut sum = 0;
    set.insert(0);
    
    for &num in nums {
        sum += num;
        if set.contains(&sum) {
            return true;
        }
        set.insert(sum);
    }
    false
}

// 6. Maximum Sum Rectangle in 2D Matrix
fn max_sum_rectangle(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut max_sum = i32::MIN;
    
    for left in 0..cols {
        let mut temp = vec![0; rows];
        
        for right in left..cols {
            for row in 0..rows {
                temp[row] += matrix[row][right];
            }
            
            let current_max = max_subarray_sum(&temp);
            max_sum = max_sum.max(current_max);
        }
    }
    max_sum
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let (mut max_ending_here, mut max_so_far) = (arr[0], arr[0]);
    for &num in arr.iter().skip(1) {
        max_ending_here = max_ending_here.max(num + max_ending_here);
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}

// 7. Find K Closest Points to the Origin
fn k_closest_points(points: Vec<(i32, i32)>, k: usize) -> Vec<(i32, i32)> {
    let mut distances: Vec<(i32, (i32, i32))> = points
        .iter()
        .map(|&(x, y)| (x * x + y * y, (x, y)))
        .collect();
    distances.sort_by_key(|&(dist, _)| dist);
    distances.into_iter().take(k).map(|(_, point)| point).collect()
}

// 8. Trapping Rain Water
fn trap_rain_water(heights: &[i32]) -> i32 {
    let (mut left, mut right) = (0, heights.len() - 1);
    let (mut max_left, mut max_right) = (0, 0);
    let mut water = 0;

    while left <= right {
        if heights[left] <= heights[right] {
            max_left = max_left.max(heights[left]);
            water += max_left - heights[left];
            left += 1;
        } else {
            max_right = max_right.max(heights[right]);
            water += max_right - heights[right];
            right -= 1;
        }
    }
    water
}

// 9. Minimum Swaps to Sort Array
fn minimum_swaps_to_sort(nums: &mut [i32]) -> i32 {
    let mut swaps = 0;
    let mut visited = vec![false; nums.len()];

    let mut indexed_nums: Vec<(usize, i32)> = nums.iter().cloned().enumerate().collect();
    indexed_nums.sort_by_key(|&(_, value)| value);

    for i in 0..nums.len() {
        if visited[i] || indexed_nums[i].0 == i {
            continue;
        }

        let mut cycle_size = 0;
        let mut j = i;

        while !visited[j] {
            visited[j] = true;
            j = indexed_nums[j].0;
            cycle_size += 1;
        }
        if cycle_size > 0 {
            swaps += cycle_size - 1;
        }
    }
    swaps
}

// 10. Find Missing and Repeating Numbers in Array
fn find_missing_and_repeating(nums: &[i32]) -> (i32, i32) {
    let mut num_counts = HashMap::new();
    let n = nums.len() as i32;

    for &num in nums {
        *num_counts.entry(num).or_insert(0) += 1;
    }

    let mut missing = 0;
    let mut repeating = 0;

    for i in 1..=n {
        match num_counts.get(&i) {
            Some(&count) if count > 1 => repeating = i,
            None => missing = i,
            _ => {}
        }
    }
    (missing, repeating)
}

fn main() {
    // Example usage:

    let arr = vec![2, 3, -2, 4];
    println!("Max Product Subarray: {}", max_product_subarray(&arr));

    let arr = vec![1,3,-1,-3,5,3,6,7];
    println!("Sliding Window Maximum: {:?}", sliding_window_maximum(&arr, 3));

    let arr = vec![1, 2, 3, 4, 5];
    println!("Longest Subarray with Sum at Most K: {}", longest_subarray_with_sum_at_most_k(&arr, 8));

    let arr = vec![5, 2, 6, 1];
    println!("Count of Smaller Numbers After Self: {:?}", count_smaller_after_self(arr.clone()));

    let arr = vec![1, 2, -3, 4, 5];
    println!("Subarray Sum Equals Zero: {}", subarray_sum_equals_zero(&arr));

    let matrix = vec![
        vec![1, 2, -1, -4, -20],
        vec![-8, -3, 4, 2, 1],
        vec![3, 8, 10, 1, 3],
        vec![-4, -1, 1, 7, -6],
    ];
    println!("Max Sum Rectangle in 2D Matrix: {}", max_sum_rectangle(matrix));

    let points = vec![(1, 3), (3, 4), (2, -1)];
    println!("K Closest Points to Origin: {:?}", k_closest_points(points, 2));

    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("Trapping Rain Water: {}", trap_rain_water(&heights));

    let mut arr = vec![4, 3, 2, 1];
    println!("Minimum Swaps to Sort Array: {}", minimum_swaps_to_sort(&mut arr));

    let arr = vec![4, 3, 6, 2, 1, 1];
    println!("Find Missing and Repeating Numbers: {:?}", find_missing_and_repeating(&arr));
}
