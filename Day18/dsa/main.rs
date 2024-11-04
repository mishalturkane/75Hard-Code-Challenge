use std::collections::HashMap;

// 1. Find the Most Frequent Element in an Array
fn find_most_frequent_element(arr: &[i32]) -> Option<i32> {
    let mut frequency = HashMap::new();
    for &num in arr {
        *frequency.entry(num).or_insert(0) += 1;
    }
    frequency.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num)
}

// 2. Find the First Unique Element in an Array
fn find_first_unique_element(arr: &[i32]) -> Option<i32> {
    let mut frequency = HashMap::new();
    for &num in arr {
        *frequency.entry(num).or_insert(0) += 1;
    }
    arr.iter().find(|&&num| frequency[&num] == 1).copied()
}

// 3. Check if Two Arrays are Equal
fn are_arrays_equal(arr1: &[i32], arr2: &[i32]) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for &num in arr1 {
        *map1.entry(num).or_insert(0) += 1;
    }
    for &num in arr2 {
        *map2.entry(num).or_insert(0) += 1;
    }

    map1 == map2
}

// 4. Group Elements by Frequency
fn group_elements_by_frequency(arr: &[i32]) -> HashMap<i32, Vec<i32>> {
    let mut frequency = HashMap::new();
    for &num in arr {
        *frequency.entry(num).or_insert(0) += 1;
    }

    let mut groups = HashMap::new();
    for (&num, &count) in &frequency {
        groups.entry(count).or_insert_with(Vec::new).push(num);
    }
    groups
}

// 5. Count Pairs with a Given Sum
fn count_pairs_with_sum(arr: &[i32], sum: i32) -> i32 {
    let mut count = 0;
    let mut map = HashMap::new();
    for &num in arr {
        let complement = sum - num;
        if let Some(&freq) = map.get(&complement) {
            count += freq;
        }
        *map.entry(num).or_insert(0) += 1;
    }
    count
}

// 6. Find Elements that Occur More Than N/K Times
fn elements_more_than_n_div_k(arr: &[i32], k: i32) -> Vec<i32> {
    let mut frequency = HashMap::new();
    for &num in arr {
        *frequency.entry(num).or_insert(0) += 1;
    }

    let threshold = arr.len() as i32 / k;
    frequency.into_iter()
        .filter(|&(_, count)| count > threshold)
        .map(|(num, _)| num)
        .collect()
}

// 7. Check if Array Contains Duplicates within K Distance
fn contains_nearby_duplicates(arr: &[i32], k: usize) -> bool {
    let mut map = HashMap::new();
    for (i, &num) in arr.iter().enumerate() {
        if let Some(&last_index) = map.get(&num) {
            if i - last_index <= k {
                return true;
            }
        }
        map.insert(num, i);
    }
    false
}

// 8. Find Subarray with Given Sum
fn find_subarray_with_sum(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    let mut current_sum = 0;
    map.insert(0, 0);

    for (i, &num) in arr.iter().enumerate() {
        current_sum += num;
        if let Some(&start) = map.get(&(current_sum - target)) {
            return Some((start, i));
        }
        map.insert(current_sum, i + 1);
    }
    None
}

// 9. Count Distinct Elements in Each Window of Size K
fn count_distinct_in_windows(arr: &[i32], k: usize) -> Vec<usize> {
    let mut map = HashMap::new();
    let mut distinct_counts = Vec::new();
    let mut distinct_count = 0;

    for i in 0..arr.len() {
        *map.entry(arr[i]).or_insert(0) += 1;
        if map[&arr[i]] == 1 {
            distinct_count += 1;
        }
        
        if i >= k {
            if let Some(count) = map.get_mut(&arr[i - k]) {
                if *count == 1 {
                    distinct_count -= 1;
                }
                *count -= 1;
            }
        }

        if i >= k - 1 {
            distinct_counts.push(distinct_count);
        }
    }
    distinct_counts
}

// 10. Find Intersection of Two Arrays
fn find_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    for &num in arr1 {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for &num in arr2 {
        if let Some(count) = map.get_mut(&num) {
            if *count > 0 {
                result.push(num);
                *count -= 1;
            }
        }
    }
    result
}

fn main() {
    let arr1 = vec![1, 2, 2, 3, 4];
    let arr2 = vec![2, 2, 3, 5];

    println!("1. Most Frequent Element: {:?}", find_most_frequent_element(&arr1));
    println!("2. First Unique Element: {:?}", find_first_unique_element(&arr1));
    println!("3. Arrays are Equal: {:?}", are_arrays_equal(&arr1, &arr2));
    println!("4. Group by Frequency: {:?}", group_elements_by_frequency(&arr1));
    println!("5. Count Pairs with Sum: {:?}", count_pairs_with_sum(&arr1, 5));
    println!("6. Elements more than N/K times: {:?}", elements_more_than_n_div_k(&arr1, 2));
    println!("7. Contains Nearby Duplicates: {:?}", contains_nearby_duplicates(&arr1, 3));
    println!("8. Subarray with Given Sum: {:?}", find_subarray_with_sum(&arr1, 6));
    println!("9. Distinct Count in Windows: {:?}", count_distinct_in_windows(&arr1, 3));
    println!("10. Intersection of Arrays: {:?}", find_intersection(&arr1, &arr2));
}
