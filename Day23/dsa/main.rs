// 1. Find the First and Last Position of an Element in a Sorted Array
fn find_first_and_last(arr: &[i32], target: i32) -> (i32, i32) {
    let first = arr.iter().position(|&x| x == target).map_or(-1, |i| i as i32);
    let last = arr.iter().rposition(|&x| x == target).map_or(-1, |i| i as i32);
    (first, last)
}

// 2. Move All Zeros to the End of the Array
fn move_zeros(arr: &mut Vec<i32>) {
    let mut non_zero_index = 0;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            arr.swap(non_zero_index, i);
            non_zero_index += 1;
        }
    }
}

// 3. Find Missing Numbers in a Sequence
fn find_missing_numbers(arr: &[i32], n: i32) -> Vec<i32> {
    let mut present = vec![false; n as usize];
    for &num in arr {
        if num <= n && num > 0 {
            present[(num - 1) as usize] = true;
        }
    }
    present.iter().enumerate().filter_map(|(i, &x)| if !x { Some((i + 1) as i32) } else { None }).collect()
}

// 4. Sort Array by Parity
fn sort_array_by_parity(arr: &mut Vec<i32>) {
    let mut even_index = 0;
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            arr.swap(even_index, i);
            even_index += 1;
        }
    }
}

// 5. Find Duplicates in an Array
fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    for &num in arr {
        if !seen.insert(num) {
            duplicates.insert(num);
        }
    }
    duplicates.into_iter().collect()
}

// 6. Subarray Sum Equals K
fn subarray_sum_equals_k(arr: &[i32], k: i32) -> i32 {
    use std::collections::HashMap;
    let mut sum = 0;
    let mut count = 0;
    let mut prefix_sum = HashMap::new();
    prefix_sum.insert(0, 1);
    for &num in arr {
        sum += num;
        if let Some(&val) = prefix_sum.get(&(sum - k)) {
            count += val;
        }
        *prefix_sum.entry(sum).or_insert(0) += 1;
    }
    count
}

// 7. Find the Longest Consecutive Sequence
fn longest_consecutive_sequence(arr: &[i32]) -> i32 {
    use std::collections::HashSet;
    let num_set: HashSet<i32> = arr.iter().cloned().collect();
    let mut longest_streak = 0;
    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;
            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            longest_streak = longest_streak.max(current_streak);
        }
    }
    longest_streak
}

// 8. Product of Array Except Self
fn product_except_self(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut result = vec![1; n];
    let mut left = 1;
    for i in 0..n {
        result[i] = left;
        left *= arr[i];
    }
    let mut right = 1;
    for i in (0..n).rev() {
        result[i] *= right;
        right *= arr[i];
    }
    result
}

// 9. Maximum Product of Three Numbers
fn max_product_of_three(arr: &[i32]) -> i32 {
    let mut sorted = arr.to_vec();
    sorted.sort();
    let n = sorted.len();
    (sorted[n - 1] * sorted[n - 2] * sorted[n - 3]).max(sorted[0] * sorted[1] * sorted[n - 1])
}

// 10. Find the Minimum Size Subarray Sum
fn min_size_subarray_sum(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut sum = 0;
    let mut min_len = i32::MAX;
    for right in 0..arr.len() {
        sum += arr[right];
        while sum >= target {
            min_len = min_len.min((right - left + 1) as i32);
            sum -= arr[left];
            left += 1;
        }
    }
    if min_len == i32::MAX { 0 } else { min_len }
}

fn main() {
    let mut array = vec![1, 2, 0, 4, 3, 5, 0, 6];
    let target = 3;

    // 1. Find the First and Last Position of an Element in a Sorted Array
    println!("1. First and Last Position of Target: {:?}", find_first_and_last(&array, target));

    // 2. Move All Zeros to the End of the Array
    move_zeros(&mut array);
    println!("2. Move Zeros to End: {:?}", array);

    // 3. Find Missing Numbers in a Sequence
    println!("3. Missing Numbers: {:?}", find_missing_numbers(&[3, 7, 1, 2, 8, 4, 5], 10));

    // 4. Sort Array by Parity
    let mut parity_array = vec![3, 1, 2, 4];
    sort_array_by_parity(&mut parity_array);
    println!("4. Sorted by Parity: {:?}", parity_array);

    // 5. Find Duplicates in an Array
    println!("5. Duplicates in Array: {:?}", find_duplicates(&[4, 3, 2, 7, 8, 2, 3, 1]));

    // 6. Subarray Sum Equals K
    println!("6. Subarrays with Sum Equals K: {:?}", subarray_sum_equals_k(&[1, 1, 1], 2));

    // 7. Find the Longest Consecutive Sequence
    println!("7. Longest Consecutive Sequence: {:?}", longest_consecutive_sequence(&[100, 4, 200, 1, 3, 2]));

    // 8. Product of Array Except Self
    println!("8. Product of Array Except Self: {:?}", product_except_self(&[1, 2, 3, 4]));

    // 9. Maximum Product of Three Numbers
    println!("9. Max Product of Three Numbers: {:?}", max_product_of_three(&[-10, -10, 5, 2]));

    // 10. Find the Minimum Size Subarray Sum
    println!("10. Min Size Subarray with Sum >= Target: {:?}", min_size_subarray_sum(&[2, 3, 1, 2, 4, 3], 7));
}
