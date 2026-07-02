use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            match hashmap.get(&num) {
                Some(count) => {
                    hashmap.insert(num, count + 1);
                },
                None => {
                    hashmap.insert(num, 1);
                    ()
                }
            };
        }

        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for (value, count) in hashmap {
            heap.push(Reverse((count, value)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }


        return heap.into_iter().map(|Reverse((_count, value))| value).collect()
    }
}
