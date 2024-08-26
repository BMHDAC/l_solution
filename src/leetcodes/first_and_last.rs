use super::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut output: Vec<i32> = Vec::with_capacity(2);
        let mut lo: i32 = 0;
        let mut hi: i32 = nums.len() as i32;

        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] == target {
                // search inside
                println!("index: {mid}");
                get_output(&mut output, mid as usize, &nums);
                if !output.is_empty() {
                    return output;
                }
            } else {
                match nums[mid as usize] > target {
                    true => {
                        hi = mid;
                    }
                    false => {
                        lo = mid + 1;
                    }
                }
            }
        }
        vec![-1, -1]
    }
}

pub fn get_output(output: &mut Vec<i32>, index: usize, input: &Vec<i32>) {
    let mut start = index;
    let mut end = index;
    for i in (0..start).rev() {
        if input[i] == input[index] {
            start = i
        }
    }
    for i in index..input.len() {
        if input[i] == input[index] {
            end = i
        }
    }
    output.push(start as i32);
    output.push(end as i32);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_1() {
//         let input = vec![5, 7, 7, 8, 8, 10];
//         let target = 8;
//         let output = search_range(input, target);
//         assert_eq!(output, vec![3, 4]);
//     }
//
//     #[test]
//     fn test_2() {
//         let input = vec![1];
//         let target = 0;
//         let output = search_range(input, target);
//         assert_eq!(output, vec![-1, -1]);
//     }
//
//     #[test]
//     fn test_3() {
//         let input = vec![1, 3];
//         let target = 1;
//         let output = search_range(input, target);
//         assert_eq!(output, vec![0, 0]);
//     }
//
//     #[test]
//     fn test_4() {
//         let input = vec![3, 3, 3];
//         let target = 3;
//         let output = search_range(input, target);
//         assert_eq!(output, vec![0, 2]);
//     }
// }
