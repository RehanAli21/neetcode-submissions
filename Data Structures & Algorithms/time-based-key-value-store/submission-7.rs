use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<(String, i32)>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap{ data: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.data.get_mut(&key) {
            Some(entry) => {
                entry.push((value.clone(), timestamp));
            },
            None => {
                self.data.insert(key.clone(), vec![(value.clone(), timestamp)]);
            },
        };
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        let _ = match self.data.get_mut(&key) {
            Some(entry) => {
                let (mut l, mut r) = (0i32, entry.len() as i32 - 1);
                let mut res = "".to_string();

                while l <= r {
                    let m = l + (r - l) / 2;

                    if entry[m as usize].1 <= timestamp {
                        res = entry[m as usize].0.clone();
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }

                return res
            },
            None => ()
        };

        "".to_string()
    }
}
