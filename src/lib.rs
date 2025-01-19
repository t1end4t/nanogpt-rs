pub mod utils;

pub fn pretty_print_2d_vector<T: std::fmt::Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        println!(
            "{:?}",
            row.iter()
                .map(|item| format!("{}", item))
                .collect::<Vec<_>>()
        );
    }
}

pub fn get_row_values_in_range(
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

pub fn get_value(matrix: &Vec<Vec<f32>>, row: usize, col: usize) -> f32 {
    // Ensure the row index is valid
    if row >= matrix.len() {
        panic!("Row index {} is out of bounds.", row);
    }

    let row_data = &matrix[row];

    // Ensure the column index is valid
    if col >= row_data.len() {
        panic!(
            "Column index {} is out of bounds for row of length {}.",
            col,
            row_data.len()
        );
    }

    row_data[col]
}
