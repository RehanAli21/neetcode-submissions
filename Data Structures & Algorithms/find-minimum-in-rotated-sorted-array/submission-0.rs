impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        return if nums.len() > 0 {nums[0]} else {0}

    }
}
