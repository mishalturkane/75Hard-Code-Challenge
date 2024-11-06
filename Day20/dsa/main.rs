// 1. Find if an Array has Two Elements with a Given Sum
fn find_two_sum(arr: &[i32], target: i32) -> bool {
    let (mut left, mut right) = (0, arr.len() - 1);
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    
    while left < right {
        let sum = sorted_arr[left] + sorted_arr[right];
        if sum == target {
            return true;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    false
}

// 2. Remove Duplicates from a Sorted Array
fn remove_duplicates(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() { return 0; }
    let mut j = 0;

    for i in 1..arr.len() {
        if arr[i] != arr[j] {
            j += 1;
            arr[j] = arr[i];
        }
    }
    j + 1
}

// 3. Move All Zeros to the End of Array
fn move_zeros(arr: &mut Vec<i32>) {
    let mut last_non_zero = 0;

    for i in 0..arr.len() {
        if arr[i] != 0 {
            arr.swap(i, last_non_zero);
            last_non_zero += 1;
        }
    }
}

// 4. Find the Pair with the Closest Sum to Zero
fn closest_sum_to_zero(arr: &[i32]) -> (i32, i32) {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    let (mut left, mut right) = (0, sorted_arr.len() - 1);
    let (mut closest_sum, mut closest_pair) = (i32::MAX, (0, 0));

    while left < right {
        let sum = sorted_arr[left] + sorted_arr[right];
        if sum.abs() < closest_sum.abs() {
            closest_sum = sum;
            closest_pair = (sorted_arr[left], sorted_arr[right]);
        }

        if sum < 0 {
            left += 1;
        } else {
            right -= 1;
        }
    }
    closest_pair
}

// 5. Check if an Array is a Palindrome
fn is_palindrome(arr: &[i32]) -> bool {
    let (mut left, mut right) = (0, arr.len() - 1);

    while left < right {
        if arr[left] != arr[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

// 6. Find the Intersection of Two Sorted Arrays
fn intersection_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut intersection = Vec::new();

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] == arr2[j] {
            if intersection.last() != Some(&arr1[i]) {
                intersection.push(arr1[i]);
            }
            i += 1;
            j += 1;
        } else if arr1[i] < arr2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    intersection
}

// 7. Merge Two Sorted Arrays
fn merge_two_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut merged = Vec::new();

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

// 8. Reverse Only the Vowels in a String
fn reverse_vowels(s: &mut Vec<char>) {
    let (mut left, mut right) = (0, s.len() - 1);
    let vowels = "aeiouAEIOU".chars().collect::<Vec<char>>();

    while left < right {
        while left < right && !vowels.contains(&s[left]) {
            left += 1;
        }
        while left < right && !vowels.contains(&s[right]) {
            right -= 1;
        }
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

// 9. Find the Longest Subarray with Equal Numbers of 0s and 1s
fn longest_equal_zeros_ones(arr: &[i32]) -> i32 {
    let mut count_map = std::collections::HashMap::new();
    count_map.insert(0, -1);
    let mut max_len = 0;
    let mut count = 0;

    for (i, &num) in arr.iter().enumerate() {
        count += if num == 0 { -1 } else { 1 };
        if let Some(&index) = count_map.get(&count) {
            max_len = max_len.max(i as i32 - index);
        } else {
            count_map.insert(count, i as i32);
        }
    }

    max_len
}

// 10. Find if there is a Subarray with Sum Zero
fn has_subarray_with_sum_zero(arr: &[i32]) -> bool {
    let mut sum = 0;
    let mut sum_set = std::collections::HashSet::new();
    sum_set.insert(0);

    for &num in arr {
        sum += num;
        if sum_set.contains(&sum) {
            return true;
        }
        sum_set.insert(sum);
    }

    false
}

fn main() {
    // Test cases
    let mut arr1 = vec![1, 3, 2, 4, 5];
    let arr2 = vec![0, 1, 0, 1, 1, 0];
    let mut string_arr = vec!['h', 'e', 'l', 'l', 'o'];

    // 1. Find Two Elements with Given Sum
    println!("1. Has Two Sum: {:?}", find_two_sum(&arr1, 6));

    // 2. Remove Duplicates from Sorted Array
    println!("2. Unique Length: {}", remove_duplicates(&mut arr1));

    // 3. Move All Zeros to the End
    let mut zeros_arr = vec![0, 1, 0, 3, 12];
    move_zeros(&mut zeros_arr);
    println!("3. After Moving Zeros: {:?}", zeros_arr);

    // 4. Pair with Closest Sum to Zero
    println!("4. Closest Sum Pair: {:?}", closest_sum_to_zero(&arr1));

    // 5. Check if Array is Palindrome
    println!("5. Is Palindrome: {:?}", is_palindrome(&[1, 2, 3, 2, 1]));

    // 6. Intersection of Two Sorted Arrays
    println!("6. Intersection: {:?}", intersection_sorted_arrays(&[1, 2, 3], &[2, 3, 4]));

    // 7. Merge Two Sorted Arrays
    println!("7. Merged Array: {:?}", merge_two_sorted_arrays(&[1, 3, 5], &[2, 4, 6]));

    // 8. Reverse Only Vowels
    reverse_vowels(&mut string_arr);
    println!("8. Reversed Vowels: {:?}", string_arr);

    // 9. Longest Subarray with Equal 0s and 1s
    println!("9. Longest Equal Subarray: {}", longest_equal_zeros_ones(&arr2));

    // 10. Subarray with Sum  zero
    println!("10. Has Subarray with Sum Zero: {:?}", has_subarray_with_sum_zero(&[-3, 2, 3, 1, 6]));
}
