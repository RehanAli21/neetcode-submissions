use std::collections::HashMap;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut counts = HashMap::new();

        for num in nums {
            match counts.get(&num) {
                Some(c) => return true,
                None => counts.entry(num).or_insert(1)
            };
        }

        return false
    }
}
