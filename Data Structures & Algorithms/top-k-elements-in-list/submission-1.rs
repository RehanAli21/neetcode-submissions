use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
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

        for i in 0i32..k {
            let mut max = (-1, 0);

            for (value, count) in hashmap.iter() {
                if *count >= max.1 {
                    max = (*value, *count);
                }
            }

            res.push(max.0);

            hashmap.remove(&max.0);
        }

        println!("{:#?}", hashmap);


        return res
    }
}
