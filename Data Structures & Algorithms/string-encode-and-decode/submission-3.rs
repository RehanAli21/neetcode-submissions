use regex::Regex;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut string: String = "".to_string();

        for s in strs {
            let str_length = s.len();

            string.push('#');
            string.push_str(&s.len().to_string());
            string.push('#');

            string.push_str(&s);
        }

        return string
    }

    pub fn decode(s: String) -> Vec<String> {
        let re = Regex::new(r"#\d+#").unwrap();

        let mut splitted_strs: Vec<_> = re.split(&s).collect();
        splitted_strs.remove(0);

        return splitted_strs.into_iter().map(String::from).collect();



        println!("{}", re);
        println!("{:#?}", splitted_strs);
        println!("{}", s);
        let mut strs: Vec<String> = vec![];

        //let mut first_time = true;
        let mut length = -1;
        let mut new_str = "".to_string();

        for c in s.chars() {
            if length < 1 {

                length = c.to_digit(10).unwrap() as i32;
                
                new_str = "".to_string();
                if c == '0' {
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
