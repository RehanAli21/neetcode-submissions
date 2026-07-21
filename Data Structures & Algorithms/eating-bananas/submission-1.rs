struct Rara;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap();
        let mut res = r;

        while l <= r {
            let mid = (l + r) / 2;

            let mut total_time: i64 = 0;

            for &p in &piles {
                total_time += (p as i64 + mid as i64 - 1) / mid as i64;
            }

            if total_time <= h as i64 {
                res = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        res
    }
}
