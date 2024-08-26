struct SnapshotArray {
    pool: Vec<Vec<(usize, i32)>>,
    snap_id: usize,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut pool = Vec::with_capacity(length as usize);
        for _ in 0..length {
            pool.push(vec![(0, 0)])
        }

        Self { snap_id: 0, pool }
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.pool[index as usize].last().unwrap().0 == self.snap_id {
            self.pool[index as usize].pop();
        }
        self.pool[index as usize].push((self.snap_id, val))
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        (self.snap_id - 1) as i32
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let nums = &self.pool[index as usize];
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid].0 == snap_id as usize {
                lo = mid + 1;
                continue;
            } else {
                match nums[mid].0 > snap_id as usize {
                    true => {
                        hi = mid - 1;
                    }
                    false => lo = mid + 1,
                }
            }
        }
        nums[lo - 1].1
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_init() {
//         let snap = SnapshotArray::new(3);
//         assert_eq!(snap.pool, vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0)]]);
//     }
//
//     #[test]
//     fn test_set() {
//         let mut snap = SnapshotArray::new(3);
//         snap.set(2, -1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//     }
//
//     #[test]
//     fn test_snap() {
//         let mut snap = SnapshotArray::new(3);
//         snap.set(2, -1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//         let snap_id = snap.snap();
//         assert_eq!(snap_id, 0);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//     }
//
//     #[test]
//     fn test_set_after_snap() {
//         let mut snap = SnapshotArray::new(3);
//         snap.set(2, -1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//         let snap_id = snap.snap();
//         assert_eq!(snap_id, 0);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//         snap.set(2, 10);
//         assert_eq!(snap.snap_id, 1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1), (1, 10)]]
//         );
//     }
//
//     #[test]
//     fn get_after_3_snap() {
//         let mut snap = SnapshotArray::new(3);
//         snap.set(2, -1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//         let snap_id = snap.snap();
//         assert_eq!(snap_id, 0);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1)]]
//         );
//         snap.set(2, 10);
//         assert_eq!(snap.snap_id, 1);
//         assert_eq!(
//             snap.pool,
//             vec![vec![(0, 0)], vec![(0, 0)], vec![(0, 0), (0, -1), (1, 10)]]
//         );
//         snap.snap();
//         snap.set(1, 1);
//         assert_eq!(
//             snap.pool,
//             vec![
//                 vec![(0, 0)],
//                 vec![(0, 0), (2, 1)],
//                 vec![(0, 0), (0, -1), (1, 10)]
//             ]
//         );
//         assert_eq!(snap.get(1, 2), 1);
//         assert_eq!(snap.get(2, 2), 10);
//     }
//
//     #[test]
//     fn leet_code() {
//         let mut snap = SnapshotArray::new(1);
//         snap.set(0, 4);
//         snap.set(0, 16);
//         snap.set(0, 13);
//         snap.snap();
//         println!("{:?}", snap.pool);
//         let output = snap.get(0, 0);
//         assert_eq!(output, 13);
//     }
// }
