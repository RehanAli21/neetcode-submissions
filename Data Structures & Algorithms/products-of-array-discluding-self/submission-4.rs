impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut outputs: Vec<i32> = vec![0; nums.len()];

        let mut prefix_value: i32 = 1;
        let mut postfix_value: i32 = 1;

        for i in 0..outputs.len() {
            if i != 0 {
                prefix_value *= nums[i - 1];
            }

            outputs[i] = prefix_value;
        }

        for i in (0..outputs.len()).rev() {

            if i != outputs.len() - 1 {
                postfix_value *= nums[i + 1];
            }

            outputs[i] *= postfix_value;
        }

        
    
        outputs
    }
}
