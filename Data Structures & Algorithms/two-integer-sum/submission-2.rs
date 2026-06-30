use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::<i32, i32>::new();
        let mut return_value = vec![0, 1];

        for (index, num) in nums.iter().enumerate() {
            let i = index as i32;
            hashmap.entry(*num).or_insert(i);
        }

        for (index, num) in nums.iter().enumerate() {
            let i = index as i32;
            
            let diff = target - num;

            match hashmap.get(&diff) {
                Some(hashmap_index) => {

                    if i == *hashmap_index {
                        continue
                    }

                    let first_index = i;
                    let second_index = *hashmap_index;

                    let smaller_index = min(first_index, second_index);
                    let larger_index = max(first_index, second_index);

                    return vec![smaller_index, larger_index]
                },
                None => ()
            }
        }

        return_value
    }
}
