use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut starting: HashSet<char> = HashSet::new();
        starting.insert('(');
        starting.insert('[');
        starting.insert('{');

        let mut ending: HashSet<char> = HashSet::new();
        ending.insert(')');
        ending.insert(']');
        ending.insert('}');

        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert(']', '[');
        pairs.insert('}', '{');

        // this is actually stack
        let mut list_of_starting: Vec<char> = vec![];

        for c in s.chars() {
            if starting.contains(&c) {
                list_of_starting.push(c);
            } else if ending.contains(&c) {
                if list_of_starting.len() < 1 {
                    return false;
                }

                let last_of_starting = list_of_starting[list_of_starting.len() - 1];


                if &last_of_starting == pairs.get(&c).unwrap() {
                    list_of_starting.pop();
                } else {
                    return false;
                }
            }
        }

        if list_of_starting.len() > 0 {
            return false;
        }

        true
    }
}
