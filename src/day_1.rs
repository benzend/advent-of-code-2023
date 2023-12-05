pub fn first_and_last_num(s: &str) -> Option<(u128, u128)> {
    let mut first_num = None;
    let mut last_num = None;

    for ch in s.chars() {
        if ch.is_digit(10) {
            if first_num.is_none() {
                first_num = Some(ch);
            }

            last_num = Some(ch);
        }
    }

    if first_num.is_none() || last_num.is_none() {
        None
    } else {
        Some((
            first_num.unwrap().to_digit(10).unwrap().into(),
            last_num.unwrap().to_digit(10).unwrap().into(),
        ))
    }
}

pub fn first_and_last_num_int_n_txt(s: &str) -> Option<(u128, u128)> {
    let first = find_first_num_int_or_txt(s);
    let last = find_last_num_int_or_txt(s);

    if first.is_none() || last.is_none() {
        None
    } else {
        Some((
            first.unwrap().to_string().parse::<u128>().unwrap().into(),
            last.unwrap().to_string().parse::<u128>().unwrap().into(),
        ))
    }
}

fn str_contains_num(s: &str) -> bool {
    s.contains("one")
        || s.contains("two")
        || s.contains("three")
        || s.contains("four")
        || s.contains("five")
        || s.contains("six")
        || s.contains("seven")
        || s.contains("eight")
        || s.contains("nine")
}

fn str_extract_num(s: &str) -> u128 {
    if s.contains("one") {
        1
    } else if s.contains("two") {
        2
    } else if s.contains("three") {
        3
    } else if s.contains("four") {
        4
    } else if s.contains("five") {
        5
    } else if s.contains("six") {
        6
    } else if s.contains("seven") {
        7
    } else if s.contains("eight") {
        8
    } else if s.contains("nine") {
        9
    } else {
        panic!("failed to find digit: {}", s);
    }
}

fn find_first_num_int_or_txt(s: &str) -> Option<String> {
    let mut word = "".to_string();
    for ch in s.chars() {
        if ch.is_digit(10) {
            return Some(ch.to_string());
        }

        word.push(ch);

        if str_contains_num(word.as_str()) {
            return Some(str_extract_num(word.as_str()).to_string());
        }
    }
    None
}

fn find_last_num_int_or_txt(s: &str) -> Option<String> {
    let mut word = "".to_string();
    for ch in s.chars().rev() {
        if ch.is_digit(10) {
            return Some(ch.to_string());
        }
        word = format!("{}{}", ch, word);

        if str_contains_num(word.as_str()) {
            return Some(str_extract_num(word.as_str()).to_string());
        }
    }
    None
}
