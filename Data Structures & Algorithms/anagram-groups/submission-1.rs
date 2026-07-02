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
        let mut groups: Vec<Vec<String>> = vec![];
        
        for s in strs {
            let mut found = false;

            for group in groups.iter_mut() {
                if group.len() > 0 {
                    let check = Solution::valid_anagram(&group[0], &s);

                    if check {
                        group.push(s.clone());
                        found = true;
                        break;
                    }
                }
            }

            if found == false {
                groups.push(vec![s.clone()]);
            }

        }


        return groups
    }
}
