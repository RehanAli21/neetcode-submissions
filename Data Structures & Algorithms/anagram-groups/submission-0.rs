use std::collections::HashMap;

impl Solution {
    pub fn valid_anagram(first_str: &str, second_str: &str) -> bool {
        if first_str.len() != second_str.len() {
            return false;
        }
        
        let mut f_hashmap = HashMap::new();
        let mut s_hashmap = HashMap::new();
        
        for i in 0..first_str.len() {
            let character = first_str.chars().nth(i).unwrap();

            match f_hashmap.get(&character) {
                Some(count) => f_hashmap.insert(character, count + 1),
                None => f_hashmap.insert(character, 1)
            };

            let character = second_str.chars().nth(i).unwrap();

            match s_hashmap.get(&character) {
                Some(count) => s_hashmap.insert(character, count + 1),
                None => s_hashmap.insert(character, 1)
            };
        }

        return s_hashmap == f_hashmap
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();  // "eat" → "aet", "tea" → "aet", "tan" → "ant"
            let key = String::from_iter(key);

            hashmap.entry(key).or_insert(vec![]).push(s);
        }

        hashmap.into_values().collect()
    }
}
