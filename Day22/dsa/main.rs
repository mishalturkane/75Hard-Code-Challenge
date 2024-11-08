use std::collections::BinaryHeap;

// 1. Find the K Largest Elements in an Array
fn find_k_largest_elements(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for &num in arr {
        heap.push(num);
    }
    heap.into_sorted_vec().into_iter().rev().take(k).collect()
}

// 2. Find the K Smallest Elements in an Array
fn find_k_smallest_elements(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for &num in arr {
        heap.push(-num); // Use negative values for a min heap
    }
    heap.into_sorted_vec().into_iter().rev().take(k).map(|x| -x).collect()
}

// 3. Kth Largest Element in an Array
fn kth_largest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    for &num in arr {
        heap.push(num);
    }
    heap.into_sorted_vec().get(k - 1).copied()
}

// 4. Kth Smallest Element in an Array
fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    for &num in arr {
        heap.push(-num);
    }
    heap.into_sorted_vec().get(k - 1).map(|x| -x)
}

// 5. Sort an Array Using Heap Sort
fn heap_sort(arr: &[i32]) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for &num in arr {
        heap.push(num);
    }
    heap.into_sorted_vec()
}

// 6. Merge K Sorted Arrays
fn merge_k_sorted_arrays(arrays: Vec<Vec<i32>>) -> Vec<i32> {
    let mut heap = BinaryHeap::new();
    for array in &arrays {
        for &num in array {
            heap.push(-num);
        }
    }
    heap.into_sorted_vec().into_iter().rev().map(|x| -x).collect()
}

// 7. Top K Frequent Elements
fn top_k_frequent_elements(arr: &[i32], k: usize) -> Vec<i32> {
    use std::collections::HashMap;
    let mut freq_map = HashMap::new();
    for &num in arr {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (num, freq) in freq_map {
        heap.push((freq, num));
    }
    heap.into_sorted_vec().into_iter().rev().take(k).map(|(_, num)| num).collect()
}

// 8. Find Median of a Data Stream
fn find_median_of_stream(arr: &[i32]) -> f64 {
    let mut min_heap = BinaryHeap::new();
    let mut max_heap = BinaryHeap::new();
    for &num in arr {
        max_heap.push(-num);
        min_heap.push(max_heap.pop().unwrap().abs());
        if min_heap.len() > max_heap.len() + 1 {
            max_heap.push(-min_heap.pop().unwrap());
        }
    }
    if min_heap.len() > max_heap.len() {
        min_heap.peek().unwrap().to_owned() as f64
    } else {
        (min_heap.peek().unwrap().to_owned() as f64 + -max_heap.peek().unwrap() as f64) / 2.0
    }
}

// 9. K Closest Points to Origin
fn k_closest_points_to_origin(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)> {
    let mut heap = BinaryHeap::new();
    for &(x, y) in points {
        let distance = x * x + y * y;
        heap.push((distance, (x, y)));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.into_sorted_vec().into_iter().map(|(_, point)| point).collect()
}

// 10. Check if a Binary Heap is a Min Heap
fn is_min_heap(arr: &[i32]) -> bool {
    let len = arr.len();
    for i in 0..=(len - 2) / 2 {
        if 2 * i + 1 < len && arr[i] > arr[2 * i + 1] {
            return false;
        }
        if 2 * i + 2 < len && arr[i] > arr[2 * i + 2] {
            return false;
        }
    }
    true
}

fn main() {
    let array = vec![3, 2, 1, 5, 6, 4];
    let points = vec![(1, 3), (2, -2), (5, 8), (0, 1)];

    // 1. Find the K Largest Elements in an Array
    println!("1. K Largest Elements: {:?}", find_k_largest_elements(&array, 2));

    // 2. Find the K Smallest Elements in an Array
    println!("2. K Smallest Elements: {:?}", find_k_smallest_elements(&array, 2));

    // 3. Kth Largest Element in an Array
    println!("3. Kth Largest Element: {:?}", kth_largest_element(&array, 2));

    // 4. Kth Smallest Element in an Array
    println!("4. Kth Smallest Element: {:?}", kth_smallest_element(&array, 2));

    // 5. Sort an Array Using Heap Sort
    println!("5. Heap Sorted Array: {:?}", heap_sort(&array));

    // 6. Merge K Sorted Arrays
    let arrays = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
    println!("6. Merged K Sorted Arrays: {:?}", merge_k_sorted_arrays(arrays));

    // 7. Top K Frequent Elements
    println!("7. Top K Frequent Elements: {:?}", top_k_frequent_elements(&array, 2));

    // 8. Find Median of a Data Stream
    println!("8. Median of Stream: {}", find_median_of_stream(&array));

    // 9. K Closest Points to Origin
    println!("9. K Closest Points to Origin: {:?}", k_closest_points_to_origin(&points, 2));

    // 10. Check if a Binary Heap is a Min Heap
    println!("10. Is Min Heap: {}", is_min_heap(&array));
}
