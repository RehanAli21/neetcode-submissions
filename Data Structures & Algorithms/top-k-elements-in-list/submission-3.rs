use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut hashmap: HashMap<i32, usize> = HashMap::new();
        let mut freq = vec![Vec::new(); n + 1];

        for num in nums {
            match hashmap.get(&num) {
                Some(count) => {
                    hashmap.insert(num, count + 1usize);
                },
                None => {
                    hashmap.insert(num, 1usize);
                    ()
                }
            };
        }

        for (&num, &count) in &hashmap {
            freq[count].push(num);
        }

        let mut res = vec![];

        for i in (1..freq.len()).rev() {
            for num in &freq[i] {
                if res.len() < k {
                    res.push(*num);
                } else {
                    return res
                }
            }
        }


        return res
    }
}
