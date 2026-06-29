use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut s_hashmap = HashMap::new();
        let mut t_hashmap = HashMap::new();

        for c in s.chars() {
            let count = s_hashmap.entry(c).or_insert(0);

            *count += 1;
        }

        for c in t.chars() {
            let count = t_hashmap.entry(c).or_insert(0);

            *count += 1;
        }


        return s_hashmap == t_hashmap
    }
}
