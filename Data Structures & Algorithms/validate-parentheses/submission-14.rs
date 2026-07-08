use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() < 2 {
            return false
        }

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

        let mut list_of_starting: Vec<char> = vec![];

        let mut has_starting = false;
        let mut has_ending = false;

        for c in s.chars() {
            if starting.contains(&c) {
                has_starting = true;
                println!("{} start", c);
                list_of_starting.push(c);
            } else if ending.contains(&c) {
                has_ending = true;
                println!("{} ending", c);
                if list_of_starting.len() < 1 {
                    return false;
                }

                let last_of_starting = list_of_starting[list_of_starting.len() - 1];

                println!("{:?} last from list", last_of_starting);
                println!("{:?} this should be inver of this", pairs.get(&c).unwrap());

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
