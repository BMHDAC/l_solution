use std::collections::HashMap;

use super::Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut h: HashMap<char, i32> = HashMap::new();
        h.insert('I', 1);
        h.insert('V', 5);
        h.insert('X', 10);
        h.insert('L', 50);
        h.insert('C', 100);
        h.insert('D', 500);
        h.insert('M', 1000);
        let mut res: i32 = *h.get(&s.chars().next().unwrap()).unwrap();
        for i in (1..s.len()).rev() {
            if h.get(&s.chars().nth(i).unwrap()).unwrap()
                > h.get(&s.chars().nth(i - 1).unwrap()).unwrap()
            {
                res += h.get(&s.chars().nth(i).unwrap()).unwrap()
                    - 2 * h.get(&s.chars().nth(i - 1).unwrap()).unwrap();
            } else {
                res += h.get(&s.chars().nth(i).unwrap()).unwrap();
            }
        }
        res
    }
}
