use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::<i32, i32>::new();;

        for (index, num) in nums.iter().enumerate() {
            let i = index as i32;
            
            let diff = target - num;

            match hashmap.get(&diff) {
                Some(hashmap_index) => {
                    return vec![*hashmap_index, i]
                },
                None => ()
            }

            hashmap.insert(*num, i);
        }

        vec![]
    }
}
