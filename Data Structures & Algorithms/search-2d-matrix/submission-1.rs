impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row_index: i32 = -1;
        let last_col_index = matrix[0].len() - 1;

        // finding row in which we have target
        for i in 0..matrix.len() {
            if matrix[i][last_col_index] == target {
                return true;
            }

            if matrix[i][last_col_index] > target {
                row_index = i as i32;
                break;
            }
        }

        println!("{}", row_index);

        if row_index == -1 {
            return false;
        }

        let arr = &matrix[row_index as usize];

        let mut l = 0i32;
        let mut r = arr.len() as i32 - 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            if arr[mid as usize] == target {
                return true;
            } else if arr[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        false

    }
}
