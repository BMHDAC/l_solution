trait Solution {}
impl dyn Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let string = Self::create_char(n);
        let v: Vec<char> = string.chars().collect();
        v[(k - 1) as usize]
    }
    fn create_char(n: i32) -> String {
        if n <= 1 {
            String::from("0")
        } else {
            Self::create_char(n - 1) + "1" + &Self::inverse(&mut Self::create_char(n - 1))
        }
    }
    fn inverse(s: &mut String) -> String {
        let s: Vec<char> = s
            .chars()
            .map(|i| match i {
                '1' => '0',
                _ => '1',
            })
            .collect();
        s.iter().rev().collect()
    }
}
