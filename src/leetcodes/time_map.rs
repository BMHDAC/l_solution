struct TimeMap {
    pool: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            pool: HashMap::default(),
        }
    }

    fn set(&mut self, key: String, val: String, timestamp: i32) {
        let entry = self.pool.entry(key).or_default();
        entry.push((timestamp, val));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let value = self.pool.get(&key);
        match value {
            Some(value) => {
                let mut lo = 0;
                let mut hi = value.len();
                let mut prev = Default::default();

                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if value[mid].0 == timestamp {
                        return value[mid].1.to_string();
                    } else {
                        match value[mid].0 < timestamp {
                            true => {
                                lo = mid + 1;
                                prev = value[mid].clone();
                            }
                            false => {
                                hi = mid;
                            }
                        }
                    }
                }
                prev.1.to_string()
            }
            None => "".to_string(),
        }
    }
}
