fn main() {
    let arr = [1, 4, 5, 6, 798, 2, 5, 7, 7, 8, 8, 8];
    match max_elemntin_array2(&arr) {
        Some(max_value) => println!("the max element is:{}", max_value),
        None => println!("The array is empty"),
    }
}

fn max_elemntin_array(arr: &[i32]) -> i32 {
    let mut max_ele = arr[0];
    for &i in arr {
        if i > max_ele {
            max_ele = i;
        }
    }
    max_ele
}

fn max_elemntin_array2(arr: &[i32]) -> Option<i32> {
    arr.iter().cloned().max()
}
