impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0i32;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if target == nums[m as usize] { 
                return m;
            }
            
            // left portion
            if nums[l as usize] <= nums[m as usize] {
                if nums[m as usize] < target || nums[l as usize] > target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            } 
            // right portion
            else {
                if nums[m as usize] > target || nums[r as usize] < target {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }
        }

        return -1;

    }
}
