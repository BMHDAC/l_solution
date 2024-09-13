fn valid_anagram(s: String, t: String) -> bool {
    let mut record: [usize; 255] = [0; 255];

    if s.len() != t.len() {
        return false;
    }

    let s = s.as_bytes();
    let t = t.as_bytes();

    for i in 0..s.len() {
        record[s[i] as usize] += 1;
        record[t[i] as usize] -= 1;
    }

    record.iter().filter(|x| **x != 0).count() == 0
}
