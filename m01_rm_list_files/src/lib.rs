pub fn frame_set(pat: &str) -> Vec<i32> {
    let mut result = Vec::new();

    let mut step: i32 = 1;
    let mut frm_range: Vec<&str>;
    let mut start: i32 = 0;
    let mut end: i32 = 10;
    let mut raw_step: Vec<&str> = Default::default();

    if pat.contains(':') {
        raw_step = pat.split(':').collect();
        step = raw_step[1].parse().unwrap();
    }
    // there is no step defined
    if raw_step.is_empty() {
        if pat.contains('-') {
            frm_range = pat.split('-').collect();
            start = frm_range[0].parse().unwrap();
            end = frm_range[1].parse().unwrap();
        }
        // step is defined
    } else {
        if raw_step[0].contains('-') {
            frm_range = raw_step[0].split('-').collect();
            start = frm_range[0].parse().unwrap();
            end = frm_range[1].parse().unwrap();
        }
    }
    for n in (start..=end).step_by(step as usize) {
        result.push(n);
    }
    return result;
}
