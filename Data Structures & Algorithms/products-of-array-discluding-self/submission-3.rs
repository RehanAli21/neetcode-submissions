impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut outputs: Vec<i32> = vec![0; nums.len()];

        let mut prefix: Vec<i32> = vec![0; nums.len()];
        let mut postfix: Vec<i32> = vec![0; nums.len()];

        let mut prefix_value: i32 = 1;
        let mut postfix_value: i32 = 1;

        for index in 0..nums.len() {
            let num = nums[index];
            if index == 0 {
                prefix[0] = num;
            } else {
                prefix[index] = prefix[index - 1] * num;
            }  


            let rev_index = (nums.len() - 1) - index;
            let num = nums[rev_index];
            
            if rev_index == (nums.len() - 1) {
                postfix[rev_index] = num
            } else {
                postfix[rev_index] = postfix[rev_index + 1] * num;
            }
        }


        println!("{:#?}", prefix);


        println!("{:#?}", postfix);


        for index in 0..outputs.len() {
            if index == 0 {
                outputs[index] = 1 * postfix[index + 1];
            } else if index == outputs.len() - 1 {
                outputs[index] = prefix[index - 1] * 1;
            } else {
                outputs[index] = prefix[index - 1] * postfix[index + 1];
            }
        }
    
        outputs
    }
}
