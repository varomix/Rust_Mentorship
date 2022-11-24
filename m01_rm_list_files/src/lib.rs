use regex::Regex;

pub fn frame_set(pat: &str) -> Result<Vec<i32>, String> {
    let mut result: Vec<i32> = Vec::new();

    let mut step: i32 = 1;
    let frm_range: Vec<&str>;
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
    Ok(result)
}

pub fn file_sequence(pat: &str) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();

    let re = Regex::new(r"\.").unwrap();
    let split_pat: Vec<&str> = re.split(pat).collect();
    let path = split_pat[0];
    let range_exp = split_pat[1];
    let extension = split_pat[2];

    let start_end: Vec<&str> = range_exp.split("-").collect();
    let start: i32 = start_end[0].parse().unwrap();
    let end: i32 = start_end[1][0..start_end[1].len() - 1].parse().unwrap();
    for n in start..=end {
        let out = format!("{}.{:04}.{}", path, n, extension);
        result.push(out);
    }

    Ok(result)
}
