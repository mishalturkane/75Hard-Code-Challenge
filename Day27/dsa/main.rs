// arraydsaquestions.rs

use std::collections::HashMap;
use std::collections::HashSet;

// 1. Find the First Unique Character in a String
fn first_unique_char(s: &str) -> Option<char> {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    s.chars().find(|&c| char_count[&c] == 1)
}

// 2. Find the Intersection of Two Arrays
fn intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let set1: HashSet<_> = arr1.iter().cloned().collect();
    let set2: HashSet<_> = arr2.iter().cloned().collect();
    set1.intersection(&set2).cloned().collect()
}

// 3. Find the Difference Between Two Arrays
fn difference(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let set2: HashSet<_> = arr2.iter().cloned().collect();
    arr1.iter().filter(|&&x| !set2.contains(&x)).cloned().collect()
}

// 4. Two Sum (Find Pair with Given Sum)
fn two_sum(arr: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut map = HashMap::new();
    for &num in arr {
        let complement = target - num;
        if map.contains_key(&complement) {
            return Some((complement, num));
        }
        map.insert(num, true);
    }
    None
}

// 5. Subarray Sum Equals K
fn subarray_sum(arr: &[i32], k: i32) -> i32 {
    let mut sum_map = HashMap::new();
    let (mut count, mut sum) = (0, 0);
    sum_map.insert(0, 1);

    for &num in arr {
        sum += num;
        if let Some(&c) = sum_map.get(&(sum - k)) {
            count += c;
        }
        *sum_map.entry(sum).or_insert(0) += 1;
    }
    count
}

// 6. Longest Consecutive Sequence
fn longest_consecutive(arr: &[i32]) -> i32 {
    let set: HashSet<_> = arr.iter().cloned().collect();
    let mut longest_streak = 0;

    for &num in &set {
        if !set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            while set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            longest_streak = longest_streak.max(current_streak);
        }
    }
    longest_streak
}

// 7. Group Anagrams
fn group_anagrams(words: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort_unstable();
        let key = sorted_word.iter().collect::<String>();
        map.entry(key).or_insert_with(Vec::new).push(word);
    }
    map.into_values().collect()
}

// 8. Find All Duplicates in an Array
fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut result = Vec::new();

    for &num in arr {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count == 2 {
            result.push(num);
        }
    }
    result
}

// 9. Top K Frequent Elements
fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut map = HashMap::new();
    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = map.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freq_vec.into_iter().take(k).map(|(num, _)| num).collect()
}

// 10. Longest Substring Without Repeating Characters
fn longest_unique_substring(s: &str) -> usize {
    let (mut map, mut max_len, mut start) = (HashMap::new(), 0, 0);
    for (i, c) in s.chars().enumerate() {
        if let Some(&idx) = map.get(&c) {
            start = start.max(idx + 1);
        }
        max_len = max_len.max(i - start + 1);
        map.insert(c, i);
    }
    max_len
}

fn main() {
    // Example usage of the functions:

    let s = "loveleetcode";
    println!("First Unique Character: {:?}", first_unique_char(s));

    let arr1 = vec![1, 2, 2, 1];
    let arr2 = vec![2, 2];
    println!("Intersection of Two Arrays: {:?}", intersection(&arr1, &arr2));

    let arr3 = vec![1, 2, 2, 3];
    let arr4 = vec![2, 4];
    println!("Difference of Two Arrays: {:?}", difference(&arr3, &arr4));

    let arr5 = vec![2, 7, 11, 15];
    println!("Two Sum: {:?}", two_sum(&arr5, 9));

    let arr6 = vec![1, 1, 1];
    println!("Subarray Sum Equals K: {}", subarray_sum(&arr6, 2));

    let arr7 = vec![100, 4, 200, 1, 3, 2];
    println!("Longest Consecutive Sequence: {}", longest_consecutive(&arr7));

    let words = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];
    println!("Group Anagrams: {:?}", group_anagrams(words));

    let arr8 = vec![4, 3, 2, 7, 8, 2, 3, 1];
    println!("Find Duplicates: {:?}", find_duplicates(&arr8));

    let nums = vec![1, 1, 1, 2, 2, 3];
    println!("Top K Frequent Elements: {:?}", top_k_frequent(nums, 2));

    let s2 = "abcabcbb";
    println!("Longest Unique Substring: {}", longest_unique_substring(s2));
}
