use std::collections::HashMap;

struct TimeMap {
    data: HashMap<String, Vec<String>>
}

impl TimeMap {
    fn new() -> Self {
        TimeMap{ data: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let timestamp = timestamp as usize;
        
        match self.data.get_mut(&key) {
            Some(entry) => {
                if entry.len() > timestamp {
                    entry[timestamp] = value.clone();
                } else {
                    for i in (entry.len()-1)..timestamp {
                        if timestamp-1 == i {
                            entry.push(value.clone());
                        } else {
                            entry.push("".to_string());
                        }
                    }
                }
            },
            None => {
                let mut new_vec = vec![];

                for i in 0..timestamp+1 {
                    if timestamp == i {
                        new_vec.push(value.clone());
                    } else {
                        new_vec.push("".to_string());
                    }
                }

                self.data.insert(key.clone(), new_vec);
            },
        };

        // println!("{:#?}", self.data);
    }

    fn get(&mut self, key: String, timestamp: i32) -> String {
        println!("{:#?}", self.data);
        let timestamp = timestamp as usize;
        
        let _ = match self.data.get_mut(&key) {
            Some(entry) => {
                if entry.len() > timestamp {
                    if entry[timestamp] == "" {
                        for i in (0..timestamp).rev() {
                            if entry[i] != "" {
                                return entry[i].clone();
                            }
                        }
                    } else {
                        return entry[timestamp].clone();
                    }
                } else {
                    return entry[entry.len() - 1].clone();
                }
            },
            None => ()
        };

        "".to_string()
    }
}
