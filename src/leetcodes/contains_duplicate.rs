fn contains_duplicate(arr: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for i in arr.iter() {
        if map.contains_key(i) {
            return true;
        }
        map.insert(*i, true);
    }
    false
}
