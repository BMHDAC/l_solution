trait Solution {}
impl dyn Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut res: Vec<String> = Vec::new();
        let phone_map = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        fn back_track(
            combination: String,
            next_digits: &str,
            phone_map: [&str; 8],
            output: &mut Vec<String>,
        ) {
            if next_digits.is_empty() {
                output.push(combination.to_string());
            } else {
                let letter = phone_map[next_digits.chars().next().unwrap() as usize - '2' as usize];
                for letter in letter.chars() {
                    let new_combination = combination.clone() + &letter.to_string();
                    back_track(new_combination, &next_digits[1..], phone_map, output)
                }
            }
        }
        back_track(String::new(), &digits, phone_map, &mut res);
        res
    }
}
