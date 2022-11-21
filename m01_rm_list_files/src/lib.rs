pub fn frame_set(pat: &str) -> Vec<i32> {
    let mut result = Vec::new();
    if pat.contains("-") {
        let frm_range: Vec<&str> = pat.split('-').collect();
        let start: i32 = frm_range[0].parse().unwrap();
        let end: i32 = frm_range[1].parse().unwrap();
        for n in start..=end {
            result.push(n);
        }
    }
    return result;
}
