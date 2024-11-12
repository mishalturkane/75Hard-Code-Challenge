// arraydsaquestions.rs

// 1. Two Sum (Sorted Array)
fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, arr.len() - 1);
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

// 2. Remove Duplicates from Sorted Array
fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    let mut unique_idx = 0;
    for i in 1..arr.len() {
        if arr[i] != arr[unique_idx] {
            unique_idx += 1;
            arr[unique_idx] = arr[i];
        }
    }
    unique_idx + 1
}

// 3. Move Zeros to End
fn move_zeros(arr: &mut Vec<i32>) {
    let (mut last_non_zero, mut i) = (0, 0);
    while i < arr.len() {
        if arr[i] != 0 {
            arr.swap(last_non_zero, i);
            last_non_zero += 1;
        }
        i += 1;
    }
}

// 4. Find Pair with Given Difference
fn find_pair_with_difference(arr: &[i32], diff: i32) -> Option<(i32, i32)> {
    let (mut left, mut right) = (0, 1);
    while right < arr.len() {
        let current_diff = arr[right] - arr[left];
        if current_diff == diff && left != right {
            return Some((arr[left], arr[right]));
        } else if current_diff < diff {
            right += 1;
        } else {
            left += 1;
        }
    }
    None
}

// 5. Sort Array by Parity
fn sort_array_by_parity(arr: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left < right {
        if arr[left] % 2 > arr[right] % 2 {
            arr.swap(left, right);
        }
        if arr[left] % 2 == 0 {
            left += 1;
        }
        if arr[right] % 2 == 1 {
            right -= 1;
        }
    }
}

// 6. Container with Most Water
fn max_area(heights: &[i32]) -> i32 {
    let (mut left, mut right, mut max_area) = (0, heights.len() - 1, 0);
    while left < right {
        let height = heights[left].min(heights[right]);
        max_area = max_area.max(height * (right - left) as i32);
        if heights[left] < heights[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_area
}

// 7. Reverse Words in a String
fn reverse_words(s: &mut Vec<char>) {
    fn reverse(s: &mut Vec<char>, start: usize, end: usize) {
        let (mut i, mut j) = (start, end);
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    reverse(s, 0, s.len() - 1);
    let mut start = 0;
    for end in 0..=s.len() {
        if end == s.len() || s[end] == ' ' {
            reverse(s, start, end - 1);
            start = end + 1;
        }
    }
}

// 8. Find Intersection of Two Sorted Arrays
fn intersection_sorted(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let (mut i, mut j, mut result) = (0, 0, Vec::new());
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] == arr2[j] {
            if result.last() != Some(&arr1[i]) {
                result.push(arr1[i]);
            }
            i += 1;
            j += 1;
        } else if arr1[i] < arr2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    result
}

// 9. Three Sum (Triplet Sum Equals Zero)
fn three_sum(arr: &mut Vec<i32>) -> Vec<(i32, i32, i32)> {
    arr.sort();
    let mut result = Vec::new();
    for i in 0..arr.len() {
        if i > 0 && arr[i] == arr[i - 1] {
            continue;
        }
        let (mut left, mut right) = (i + 1, arr.len() - 1);
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            if sum == 0 {
                result.push((arr[i], arr[left], arr[right]));
                left += 1;
                right -= 1;
                while left < right && arr[left] == arr[left - 1] {
                    left += 1;
                }
                while left < right && arr[right] == arr[right + 1] {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result
}

// 10. Find Longest Subarray with Sum Less Than or Equal to K
fn longest_subarray_with_sum(arr: &[i32], k: i32) -> usize {
    let (mut left, mut right, mut sum, mut max_len) = (0, 0, 0, 0);
    while right < arr.len() {
        sum += arr[right];
        while sum > k {
            sum -= arr[left];
            left += 1;
        }
        max_len = max_len.max(right - left + 1);
        right += 1;
    }
    max_len
}

fn main() {
    // Example usage of some functions
    let mut arr1 = vec![2, 7, 11, 15];
    println!("Two Sum Sorted: {:?}", two_sum_sorted(&arr1, 9));

    let mut arr2 = vec![1, 1, 2, 3, 3];
    println!("Remove Duplicates: New Length = {}", remove_duplicates(&mut arr2));
    println!("Array after removing duplicates: {:?}", arr2);

    let mut arr3 = vec![0, 1, 0, 3, 12];
    move_zeros(&mut arr3);
    println!("Array after moving zeros: {:?}", arr3);

    let mut arr4 = vec![1, 5, 9, 13];
    println!("Pair with given difference: {:?}", find_pair_with_difference(&arr4, 4));

    let mut arr5 = vec![3, 1, 2, 4];
    sort_array_by_parity(&mut arr5);
    println!("Sorted by parity: {:?}", arr5);

    let arr6 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Max area: {}", max_area(&arr6));

    let mut s = "the sky is blue".chars().collect::<Vec<char>>();
    reverse_words(&mut s);
    println!("Reversed words: {:?}", s.iter().collect::<String>());

    let arr7 = vec![1, 2, 2, 3, 4];
    let arr8 = vec![2, 2, 3, 5];
    println!("Intersection of two sorted arrays: {:?}", intersection_sorted(&arr7, &arr8));

    let mut arr9 = vec![-1, 0, 1, 2, -1, -4];
    println!("Three Sum results: {:?}", three_sum(&mut arr9));

    let arr10 = vec![1, 2, 3, 4, 5];
    println!("Longest subarray with sum <= 8: {}", longest_subarray_with_sum(&arr10, 8));
}
