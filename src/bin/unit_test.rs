/// Retrieves a sub-vector from a specific row of a 2D matrix.
///
/// # Arguments
/// * `matrix` - A reference to a 2D matrix represented as `Vec<Vec<f32>>`.
/// * `row` - The index of the row to retrieve the values from.
/// * `col_range` - The range of column indices to extract (start..end).
///
/// # Returns
/// A `Vec<f32>` containing the values in the specified row and column range.
///
/// # Panics
/// Panics if the row index is out of bounds or the column range is invalid.
fn get_row_values_in_range(
    matrix: &Vec<Vec<f32>>,
    row: usize,
    col_range: std::ops::Range<usize>,
) -> Vec<f32> {
    // Ensure the row index is valid
    if row >= matrix.len() {
        panic!("Row index {} is out of bounds.", row);
    }

    let row_data = &matrix[row];

    // Ensure the column range is valid
    if col_range.start >= row_data.len()
        || col_range.end > row_data.len()
        || col_range.start >= col_range.end
    {
        panic!(
            "Column range {:?} is invalid for row of length {}.",
            col_range,
            row_data.len()
        );
    }

    row_data[col_range.clone()].to_vec()
}

fn main() {
    // Example usage
    let matrix: Vec<Vec<f32>> = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![6.0, 7.0, 8.0, 9.0, 10.0],
        vec![11.0, 12.0, 13.0, 14.0, 15.0],
    ];

    let row = 1;
    let col_range = 1..4;

    let result = get_row_values_in_range(&matrix, row, col_range);
    println!("Values: {:?}", result); // Output: [7.0, 8.0, 9.0]
}
