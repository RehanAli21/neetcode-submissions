impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut water_volume:i32 = 0;

        let mut left = 0;
        let mut right = heights.len() - 1;

        while left < right {
            let temp = heights[left].min(heights[right]) * (right as i32 - left as i32);

            if temp > water_volume {
                water_volume = temp;
            }

            if heights[left] <= heights[right] {
                left += 1;
            } else {
                right -= 1;
            } 
            
        }

        water_volume

    }
}
