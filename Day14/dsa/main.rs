// arraydsaquestions.rs

fn question_1_find_largest(arr: &[i32]) -> Option<i32> {
    arr.iter().cloned().max()
}

fn question_2_find_second_largest(arr: &[i32]) -> Option<i32> {
    let mut unique = arr.to_vec();
    unique.sort_unstable();
    unique.dedup();
    unique.get(unique.len().wrapping_sub(2)).cloned()
}

fn question_3_is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn question_4_reverse_array(arr: &mut [i32]) {
    arr.reverse();
}

fn question_5_find_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    let mut duplicates = vec![];
    for &num in arr {
        if !seen.insert(num) {
            duplicates.push(num);
        }
    }
    duplicates
}

fn question_6_rotate_array(arr: &mut [i32], k: usize) {
    let len = arr.len();
    let k = k % len;
    arr.reverse();
    arr[..k].reverse();
    arr[k..].reverse();
}

fn question_7_move_zeros_to_end(arr: &mut [i32]) {
    let mut non_zero_index = 0;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            arr.swap(non_zero_index, i);
            non_zero_index += 1;
        }
    }
}

fn question_8_find_missing_number(arr: &[i32], n: i32) -> i32 {
    let total_sum: i32 = (n * (n + 1)) / 2;
    let arr_sum: i32 = arr.iter().sum();
    total_sum - arr_sum
}

fn question_9_count_occurrences(arr: &[i32], target: i32) -> usize {
    arr.iter().filter(|&&x| x == target).count()
}

fn question_10_find_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let set1: std::collections::HashSet<_> = arr1.iter().cloned().collect();
    arr2.iter().cloned().filter(|x| set1.contains(x)).collect()
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let arr2 = [4, 6, 5, 9, 3];

    println!("1. Largest element: {:?}", question_1_find_largest(&arr));
    println!("2. Second largest element: {:?}", question_2_find_second_largest(&arr));
    println!("3. Is array sorted?: {:?}", question_3_is_sorted(&arr));
    
    let mut arr_copy = arr.clone();
    question_4_reverse_array(&mut arr_copy);
    println!("4. Reversed array: {:?}", arr_copy);

    println!("5. Duplicates: {:?}", question_5_find_duplicates(&arr));
    
    let mut arr_rot = arr.clone();
    question_6_rotate_array(&mut arr_rot, 3);
    println!("6. Array rotated by 3 positions: {:?}", arr_rot);

    let mut arr_zeros = [0, 1, 0, 3, 12];
    question_7_move_zeros_to_end(&mut arr_zeros);
    println!("7. Array with zeros moved to end: {:?}", arr_zeros);

    let arr_missing = [1, 2, 4, 5, 6];
    println!("8. Missing number in sequence 1 to 6: {:?}", question_8_find_missing_number(&arr_missing, 6));

    println!("9. Occurrences of '5' in array: {:?}", question_9_count_occurrences(&arr, 5));

    println!("10. Intersection of arrays: {:?}", question_10_find_intersection(&arr, &arr2));
}
