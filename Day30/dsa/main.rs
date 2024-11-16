// arraydsaquestions.rs

use std::collections::{HashMap, HashSet};

// 1. Two Sum
fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return Some((index, i));
        }
        map.insert(num, i);
    }
    None
}

// 2. Subarray Sum Equals K
fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;
    let mut prefix_sum = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);

    for num in nums {
        prefix_sum += num;
        if let Some(&freq) = map.get(&(prefix_sum - k)) {
            count += freq;
        }
        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

// 3. Longest Consecutive Sequence
fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<_> = nums.iter().cloned().collect();
    let mut longest = 0;

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current = num;
            let mut streak = 1;

            while num_set.contains(&(current + 1)) {
                current += 1;
                streak += 1;
            }

            longest = longest.max(streak);
        }
    }

    longest
}

// 4. Group Anagrams
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let key: String = chars.into_iter().collect();
        map.entry(key).or_default().push(s);
    }
    map.into_values().collect()
}

// 5. Top K Frequent Elements
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = map.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freq_vec.iter().take(k as usize).map(|&(num, _)| num).collect()
}

// 6. Minimum Index Sum of Two Lists
fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    for (i, restaurant) in list1.iter().enumerate() {
        map.insert(restaurant, i);
    }
    let mut min_index_sum = usize::MAX;
    let mut result = vec![];

    for (j, restaurant) in list2.iter().enumerate() {
        if let Some(&i) = map.get(restaurant) {
            let index_sum = i + j;
            if index_sum < min_index_sum {
                min_index_sum = index_sum;
                result = vec![restaurant.clone()];
            } else if index_sum == min_index_sum {
                result.push(restaurant.clone());
            }
        }
    }

    result
}

// 7. Isomorphic Strings
fn is_isomorphic(s: String, t: String) -> bool {
    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for (i, (cs, ct)) in s.chars().zip(t.chars()).enumerate() {
        if map_s.insert(cs, i) != map_t.insert(ct, i) {
            return false;
        }
    }

    true
}

// 8. Count Subarrays with Equal Zeroes and Ones
fn count_subarrays_equal_zeroes_ones(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;
    let mut prefix_sum = 0;
    map.insert(0, 1);

    for num in nums {
        prefix_sum += if num == 0 { -1 } else { 1 };
        if let Some(&freq) = map.get(&prefix_sum) {
            count += freq;
        }
        *map.entry(prefix_sum).or_insert(0) += 1;
    }

    count
}

// 9. Find All Anagrams in a String
fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut result = vec![];
    let mut p_count = HashMap::new();
    let mut s_count = HashMap::new();
    let p_len = p.len();

    for c in p.chars() {
        *p_count.entry(c).or_insert(0) += 1;
    }

    for (i, c) in s.chars().enumerate() {
        *s_count.entry(c).or_insert(0) += 1;
        if i >= p_len {
            let left_char = s.chars().nth(i - p_len).unwrap();
            if let Some(count) = s_count.get_mut(&left_char) {
                *count -= 1;
                if *count == 0 {
                    s_count.remove(&left_char);
                }
            }
        }
        if s_count == p_count {
            result.push((i + 1 - p_len) as i32);
        }
    }

    result
}

// 10. Happy Number
fn is_happy(mut n: i32) -> bool {
    let mut seen = HashSet::new();

    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = n.to_string().chars().map(|c| c.to_digit(10).unwrap().pow(2)).sum();
    }

    n == 1
}

fn main() {
    // Example usage:
    let nums = vec![2, 7, 11, 15];
    println!("Two Sum: {:?}", two_sum(nums, 9));

    let nums2 = vec![1, 1, 1];
    println!("Subarray Sum Equals K: {}", subarray_sum(nums2, 2));

    let nums3 = vec![100, 4, 200, 1, 3, 2];
    println!("Longest Consecutive Sequence: {}", longest_consecutive(nums3));

    let strs = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    println!("Group Anagrams: {:?}", group_anagrams(strs));

    let nums4 = vec![1, 1, 1, 2, 2, 3];
    println!("Top K Frequent Elements: {:?}", top_k_frequent(nums4, 2));
}
