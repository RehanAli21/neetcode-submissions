impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        let n = nums.len();

        for i in 0..n {
            let a = nums[i];

            if a > 0 {
                break;
            }

            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = a + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.push(vec![a, nums[left], nums[right]]);

                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }

        return result
    }
}
