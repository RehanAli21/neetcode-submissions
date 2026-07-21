impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row_index: i32 = -1;
        let mut l = 0i32;
        let mut r = matrix.len() as i32 - 1;
        let last_col_index = matrix[0].len() - 1;

        // finding row in which we have target
        while l <= r {
            let mid = l + (r - l) / 2;

            let first =  matrix[mid as usize][0];
            let last = matrix[mid as usize][last_col_index];

            if first == target || last == target {
                return true;
            } else if first < target && last > target {
                row_index = mid as i32;
                break;
            } else if first > target {
                r = mid - 1;
            } else {
                l = mid + 1;
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
