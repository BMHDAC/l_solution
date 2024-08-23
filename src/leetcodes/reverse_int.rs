use super::Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn reverse_inner(mut x: i32) -> Option<i32> {
            let is_neg = if x < 0 {
                x = x.checked_abs()?;
                -1
            } else {
                1
            };
            let x_vec: Vec<i32> = x
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();

            let mut x_rev: i32 = 0;
            for (i, &item) in x_vec.iter().enumerate() {
                x_rev = x_rev.checked_add(item.checked_mul(10_i32.pow(i as u32))?)?;
            }
            x_rev.checked_mul(is_neg)
        }
        reverse_inner(x).unwrap_or(0)
    }
}
