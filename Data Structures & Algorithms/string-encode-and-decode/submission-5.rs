impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let cap: usize = strs.iter().map(|s| s.len() + 3).sum();
        let mut encoded = String::with_capacity(cap);
        for s in &strs {
            encoded.push_str(&s.len().to_string()); // <len>#<content>
            encoded.push('#');
            encoded.push_str(s);
        }
        encoded
    }

    pub fn decode(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            // read the length by accumulating digits until '#'
            let mut len = 0usize;
            while bytes[i] != b'#' {
                len = len * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            i += 1; // skip the '#'

            // take exactly `len` bytes of content
            let end = i + len;
            result.push(s[i..end].to_string());
            i = end;
        }
        result
    }
}