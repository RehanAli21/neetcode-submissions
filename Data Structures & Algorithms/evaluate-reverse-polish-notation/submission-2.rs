impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        if tokens.len() < 2 {
            match tokens[0].parse::<i32>() {
                Ok(val) => {
                    return val;
                },
                Err(err) => {
                    return 0;
                }
            }
        }

        let mut res: i32 = 0;
        let mut stack: Vec<i32> = vec![];
        
        for token in tokens {
            let mut op = "".to_string();
            let int = match token.parse::<i32>() {
                Ok(val) => val,
                Err(err) => {
                    op = token;
                    0
                }
            };

            if op == "" {
                stack.push(int);
                continue;
            }

            let second_value = stack.pop().unwrap_or(0);
            let first_value = stack.pop().unwrap_or(0);

            println!("{}", first_value);
            println!("{}", second_value);
            println!("{}", op);

            res = match op.as_str() {
                "+" => first_value + second_value,
                "-" => first_value - second_value,
                "*" => first_value * second_value,
                "/" => first_value / second_value,
                _ => 0
            };

            println!("{}", res);
            stack.push(res)
        }

        res
    }
}
