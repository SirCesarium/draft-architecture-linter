use std::str::Chars;

pub fn handle_block_comment_end(
    chars: &mut Chars,
    result: &mut String,
    in_block_comment: &mut bool,
    block_delimiters: Option<(&str, &str)>,
    current: char,
    extension: &str,
) -> bool {
    if let Some((_, end)) = block_delimiters {
        let end_chars: Vec<char> = end.chars().collect();
        if current == end_chars[0] {
            let mut match_end = true;
            let mut preview = chars.clone();
            for &expected in end_chars.iter().skip(1) {
                if preview.next() != Some(expected) {
                    match_end = false;
                    break;
                }
            }
            if match_end {
                for _ in 1..end_chars.len() {
                    chars.next();
                }
                *in_block_comment = false;
                if extension == "jsx" || extension == "tsx" {
                    consume_jsx_closing_brace(chars);
                }
                return true;
            }
        }
    }
    if current == '\n' {
        result.push('\n');
    }
    false
}

pub fn consume_jsx_closing_brace(chars: &mut Chars) {
    let lookahead = chars.clone();
    for nc in lookahead {
        if nc.is_whitespace() {
            continue;
        }
        if nc == '}' {
            for to_consume in chars.by_ref() {
                if to_consume == '}' {
                    break;
                }
            }
        }
        break;
    }
}

pub fn try_handle_jsx_comment(chars: &mut Chars, in_block_comment: &mut bool) -> bool {
    let mut preview = chars.clone();
    let mut found = false;
    while let Some(nc) = preview.next() {
        if nc.is_whitespace() {
            continue;
        }
        if nc == '/' && preview.next() == Some('*') {
            found = true;
        }
        break;
    }

    if found {
        while let Some(nc) = chars.clone().next() {
            if nc == '/' {
                break;
            }
            chars.next();
        }
        chars.next();
        chars.next();
        *in_block_comment = true;
        true
    } else {
        false
    }
}

pub fn handle_comment_start(
    chars: &mut Chars,
    in_block_comment: &mut bool,
    in_line_comment: &mut bool,
    block_delimiters: Option<(&str, &str)>,
    line_prefix: Option<&str>,
    current: char,
    aggressive: bool,
) -> bool {
    if let Some((start, _)) = block_delimiters {
        let s_chars: Vec<char> = start.chars().collect();
        if current == s_chars[0] {
            let mut match_start = true;
            let mut preview = chars.clone();
            for &expected in s_chars.iter().skip(1) {
                if preview.next() != Some(expected) {
                    match_start = false;
                    break;
                }
            }

            if match_start {
                let is_doc = start == "/*" && chars.clone().next() == Some('*');
                if aggressive || !is_doc {
                    for _ in 1..s_chars.len() {
                        chars.next();
                    }
                    *in_block_comment = true;
                    return true;
                }
            }
        }
    }

    if let Some(prefix) = line_prefix {
        let p_chars: Vec<char> = prefix.chars().collect();
        if current == p_chars[0] {
            let mut match_line = true;
            let mut preview = chars.clone();
            for &expected in p_chars.iter().skip(1) {
                if preview.next() != Some(expected) {
                    match_line = false;
                    break;
                }
            }

            if match_line {
                let is_doc = prefix == "//" && {
                    let next = chars.clone().next();
                    next == Some('/') || next == Some('!')
                };
                if aggressive || !is_doc {
                    for _ in 1..p_chars.len() {
                        chars.next();
                    }
                    *in_line_comment = true;
                    return true;
                }
            }
        }
    }
    false
}
