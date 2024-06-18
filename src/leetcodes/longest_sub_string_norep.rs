use std::cmp::max;
trait Solution {}
impl dyn Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_index: Vec<i32> = vec![-1; 256];
        let mut i = 0;
        let mut res = 0;
        for c in s.chars().enumerate() {
            i = max(i, last_index[c.1 as usize] + 1);
            res = max(res, c.0 as i32 - i + 1);
            last_index[c.1 as usize] = c.0 as i32;
        }
        res
    }
}
