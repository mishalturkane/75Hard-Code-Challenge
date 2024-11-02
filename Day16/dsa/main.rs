// arraydsaquestions.rs

fn question_1_find_largest(matrix: &[Vec<i32>]) -> Option<i32> {
    matrix.iter().flat_map(|row| row.iter()).cloned().max()
}

fn question_2_row_sums(matrix: &[Vec<i32>]) -> Vec<i32> {
    matrix.iter().map(|row| row.iter().sum()).collect()
}

fn question_3_transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|j| (0..rows).map(|i| matrix[i][j]).collect())
        .collect()
}

fn question_4_column_sums(matrix: &[Vec<i32>]) -> Vec<i32> {
    let cols = matrix[0].len();
    (0..cols).map(|j| matrix.iter().map(|row| row[j]).sum()).collect()
}

fn question_5_are_equal(matrix1: &[Vec<i32>], matrix2: &[Vec<i32>]) -> bool {
    matrix1 == matrix2
}

fn question_6_row_with_max_sum(matrix: &[Vec<i32>]) -> Option<usize> {
    matrix.iter().map(|row| row.iter().sum::<i32>()).enumerate().max_by_key(|&(_, sum)| sum).map(|(i, _)| i)
}

fn question_7_diagonal_sums(matrix: &[Vec<i32>]) -> (i32, i32) {
    let n = matrix.len();
    let primary_diagonal_sum: i32 = (0..n).map(|i| matrix[i][i]).sum();
    let secondary_diagonal_sum: i32 = (0..n).map(|i| matrix[i][n - i - 1]).sum();
    (primary_diagonal_sum, secondary_diagonal_sum)
}

fn question_8_spiral_order(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    if matrix.is_empty() { return result; }

    let (mut top, mut bottom) = (0, matrix.len());
    let (mut left, mut right) = (0, matrix[0].len());

    while top < bottom && left < right {
        for i in left..right {
            result.push(matrix[top][i]);
        }
        top += 1;

        for i in top..bottom {
            result.push(matrix[i][right - 1]);
        }
        right -= 1;

        if top < bottom {
            for i in (left..right).rev() {
                result.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
        }

        if left < right {
            for i in (top..bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }
    result
}

fn question_9_rotate_90(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let n = matrix.len();
    (0..n).map(|i| (0..n).map(|j| matrix[n - j - 1][i]).collect()).collect()
}

fn question_10_is_symmetric(matrix: &[Vec<i32>]) -> bool {
    matrix == &question_3_transpose(matrix)
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];

    println!("1. Largest element: {:?}", question_1_find_largest(&matrix));
    println!("2. Row sums: {:?}", question_2_row_sums(&matrix));
    println!("3. Transposed matrix: {:?}", question_3_transpose(&matrix));
    println!("4. Column sums: {:?}", question_4_column_sums(&matrix));

    let matrix2 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    println!("5. Matrices are equal: {:?}", question_5_are_equal(&matrix, &matrix2));

    println!("6. Row with max sum: {:?}", question_6_row_with_max_sum(&matrix));
    println!("7. Diagonal sums (primary, secondary): {:?}", question_7_diagonal_sums(&matrix));
    println!("8. Spiral order: {:?}", question_8_spiral_order(&matrix));
    println!("9. Matrix rotated by 90 degrees: {:?}", question_9_rotate_90(&matrix));
    println!("10. Matrix is symmetric: {:?}", question_10_is_symmetric(&matrix));
}
