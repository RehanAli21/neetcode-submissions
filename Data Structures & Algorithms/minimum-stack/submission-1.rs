use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MinStack {
    values: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            values: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        let mut current_min = val;

        if self.values.len() > 0 {
            let last_min = self.values[self.values.len() - 1].1;

            if last_min < current_min {
                current_min = last_min;
            } 
        }

        self.values.push((val, current_min));
    }

    pub fn pop(&mut self) {
        self.values.pop();
        ()
    }

    pub fn top(&self) -> i32 {
        self.values[self.values.len() - 1].0
    }

    pub fn get_min(&self) -> i32 {
        self.values[self.values.len() - 1].1
    }
}
