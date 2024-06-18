use std::collections::HashMap;
trait Solution {}
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut starts = Vec::new();
        let mut start_with_index = HashMap::new();
        for (index, interval) in intervals.iter().enumerate() {
            start_with_index.insert(interval[0], index as i32);
            starts.push(interval[0]);
        }
        starts.sort_unstable();

        let mut res = Vec::new();

        for interval in intervals.into_iter() {
            let end = interval[1];
            let mut left = 0;
            let mut right = starts.len();

            while left < right {
                let mid = left + (right - left) / 2;
                if starts[mid] < end {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left < starts.len() {
                res.push(start_with_index[&starts[left]]);
            } else {
                res.push(-1);
            }
        }
        return res;
    }
}
