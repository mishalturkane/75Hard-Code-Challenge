// 1. Sort an Array in Ascending Order
fn sort_array_ascending(arr: &mut [i32]) {
    arr.sort();
}

// 2. Sort an Array in Descending Order
fn sort_array_descending(arr: &mut [i32]) {
    arr.sort_by(|a, b| b.cmp(a));
}

// 3. Find the Kth Smallest Element in an Array
fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr.get(k - 1).copied()
}

// 4. Find the Kth Largest Element in an Array
fn kth_largest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_by(|a, b| b.cmp(a));
    sorted_arr.get(k - 1).copied()
}

// 5. Sort an Array of 0s, 1s, and 2s (Dutch National Flag Problem)
fn sort_0_1_2(arr: &mut [i32]) {
    let (mut low, mut mid, mut high) = (0, 0, arr.len() - 1);

    while mid <= high {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => mid += 1,
            2 => {
                arr.swap(mid, high);
                high -= 1;
            }
            _ => {}
        }
    }
}

// 6. Merge Two Sorted Arrays
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

// 7. Sort an Array by Parity (Even elements first, then odd)
fn sort_by_parity(arr: &mut [i32]) {
    arr.sort_by_key(|&x| x % 2);
}

// 8. Find the Intersection of Two Sorted Arrays
fn sorted_array_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
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

// 9. Find the Union of Two Sorted Arrays
fn sorted_array_union(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut union = Vec::new();

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] == arr2[j] {
            if union.last() != Some(&arr1[i]) {
                union.push(arr1[i]);
            }
            i += 1;
            j += 1;
        } else if arr1[i] < arr2[j] {
            union.push(arr1[i]);
            i += 1;
        } else {
            union.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        union.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        union.push(arr2[j]);
        j += 1;
    }

    union
}

// 10. Sort an Array by Absolute Values
fn sort_by_absolute_values(arr: &mut [i32]) {
    arr.sort_by_key(|&x| x.abs());
}

fn main() {
    let mut arr = vec![3, -1, 4, 1, 5, -9, 2, -6];
    let arr1 = vec![1, 3, 5, 7];
    let arr2 = vec![2, 3, 6, 7, 8];
    let mut parity_arr = vec![3, 1, 2, 4];

    // 1. Sort in Ascending Order
    sort_array_ascending(&mut arr);
    println!("1. Sorted Ascending: {:?}", arr);

    // 2. Sort in Descending Order
    sort_array_descending(&mut arr);
    println!("2. Sorted Descending: {:?}", arr);

    // 3. Kth Smallest Element
    println!("3. 2nd Smallest Element: {:?}", kth_smallest_element(&arr, 2));

    // 4. Kth Largest Element
    println!("4. 2nd Largest Element: {:?}", kth_largest_element(&arr, 2));

    // 5. Sort 0, 1, 2 Array
    let mut zero_one_two_arr = vec![0, 1, 2, 1, 0, 2, 1];
    sort_0_1_2(&mut zero_one_two_arr);
    println!("5. Sorted 0, 1, 2 Array: {:?}", zero_one_two_arr);

    // 6. Merge Two Sorted Arrays
    println!("6. Merged Sorted Arrays: {:?}", merge_sorted_arrays(&arr1, &arr2));

    // 7. Sort by Parity
    sort_by_parity(&mut parity_arr);
    println!("7. Sorted by Parity: {:?}", parity_arr);

    // 8. Intersection of Two Sorted Arrays
    println!("8. Intersection of Sorted Arrays: {:?}", sorted_array_intersection(&arr1, &arr2));

    // 9. Union of Two Sorted Arrays
    println!("9. Union of Sorted Arrays: {:?}", sorted_array_union(&arr1, &arr2));

    // 10. Sort by Absolute Values
    sort_by_absolute_values(&mut arr);
    println!("10. Sorted by Absolute Values: {:?}", arr);
}
