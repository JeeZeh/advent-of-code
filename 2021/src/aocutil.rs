pub fn rot90<T: Copy>(vec: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let new_col_len = vec.len();
    let new_row_len = vec.first().unwrap().len();
    let mut new_vec: Vec<Vec<T>> = Vec::with_capacity(new_row_len);

    for x in 0..new_col_len {
        let mut row: Vec<T> = Vec::with_capacity(new_col_len);
        for y in 0..new_row_len {
            row.push(vec[(new_col_len - 1) - y][x]);
        }
        new_vec.push(row);
    }

    new_vec
}
