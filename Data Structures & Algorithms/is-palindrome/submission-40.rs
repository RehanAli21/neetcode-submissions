impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();

        let (mut left, mut right) = (0i32, s.len() as i32 - 1);

        while left < right {
            while left < right && !s[left as usize].is_ascii_alphanumeric() {
                left += 1;
            }

            while right > left && !s[right as usize].is_ascii_alphanumeric() {
                right -= 1;
            }

            if s[left as usize].to_ascii_lowercase() != s[right as usize].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true

    }
}
