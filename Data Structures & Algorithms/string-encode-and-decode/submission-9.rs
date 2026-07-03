impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut string: String = "".to_string();

        for s in strs {
            let str_length = s.len();

            string.push_str(&s.len().to_string());
            string.push('#');

            string.push_str(&s);
        }

        return string
    }

    pub fn decode(s: String) -> Vec<String> {
        println!("{}", s);
        let mut strs: Vec<String> = vec![];

        //let mut first_time = true;
        let mut length = -1;
        let mut new_str = "".to_string();
        let mut length_value: String = "".to_string();


        for c in s.chars() {
            if length < 1 {
                if c != '#' {
                    length_value.push(c);
                } else if c == '#' {
                    length = length_value.trim().parse().unwrap();
                    length_value = "".to_string();
                    continue;
                }

                

                
                new_str = "".to_string();
                if c == '0' && length_value.len() < 2 {
                    strs.push(new_str.clone());
                    continue
                }

            } else {
                new_str.push(c);

                length -= 1;

                if length < 1 {
                    strs.push(new_str.clone());
                }
            }
        }

        println!("{:#?}", strs);
        return strs
    }
}
