pub fn normalize_whitespace(content: &str) -> String {
    let mut res = String::with_capacity(content.len());
    let mut last_was_empty = false;
    for line in content.lines() {
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            if !last_was_empty {
                res.push('\n');
                last_was_empty = true;
            }
        } else {
            if !res.is_empty() {
                res.push('\n');
            }
            res.push_str(trimmed);
            last_was_empty = false;
        }
    }
    res.trim().to_string()
}
