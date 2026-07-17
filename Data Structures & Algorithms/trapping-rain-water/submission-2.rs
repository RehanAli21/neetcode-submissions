impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 1 {
            return 0;
        }

        let mut l = 0;
        let mut r = height.len() - 1;
        
        let mut max_left: i32 = height[l];
        let mut max_right: i32 = height[r];

        let mut res: i32 = 0;

        while l < r {
            if max_left < max_right {
                l += 1;
                max_left = max_left.max(height[l]);
                res += max_left - height[l];
            } else {
                r -= 1;
                max_right = max_right.max(height[r]);
                res += max_right - height[r];
            }
        }

        res
    }
}
