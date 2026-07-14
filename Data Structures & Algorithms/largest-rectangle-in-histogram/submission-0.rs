impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut stack: Vec<(usize, i32)> = vec![];

        for (i, &h) in heights.iter().enumerate() {
            let mut start = i;

            while let Some(&(index, height)) = stack.last() {
                if height > h {
                    stack.pop();
                    max_area = max_area.max(height * (i - index) as i32);
                    start = index;
                } else {
                    break;
                }
            }
            
            stack.push((start, h));
        }

        let n = heights.len();
        
        for &(index, height) in &stack {
            max_area = max_area.max(height * (n - index) as i32);
        }

        max_area

    }
}
