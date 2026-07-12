impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, i32)> = vec![];

        for (index, temp) in temperatures.iter().enumerate() {
            while !stack.is_empty() && *temp > stack[stack.len()-1].0 {
                let last_index = stack[stack.len()-1].1 as usize;
                stack.pop();
                res[last_index] = (index - last_index) as i32;
            }

            stack.push((*temp, index as i32));
        } 
        
        return res
    }
}
