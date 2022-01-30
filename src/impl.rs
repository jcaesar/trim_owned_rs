#![allow(dead_code)]

pub fn naive(s: String) -> String {
    s.trim().to_owned()
}

pub fn naive_opt(s: String) -> String {
    let t = s.trim();
    if t.len() == s.len() {
        s
    } else {
        t.to_string()
    }
}

pub fn safe(trim: String) -> String {
    let end_trimmed = trim.trim_end().len();
    if end_trimmed == 0 {
        return String::new(); // is guaranteed to not allocate. And you'll have to dealloc the empty string at some point...
    }
    let start_trimmed = trim.len() - trim.trim_start().len();

    let mut bytes = trim.into_bytes();
    if start_trimmed != 0 {
        bytes.copy_within(start_trimmed..end_trimmed, 0);
    }

    bytes.truncate(end_trimmed - start_trimmed);
    String::from_utf8(bytes).unwrap()
}

pub fn middle(trim: String) -> String {
    let end_trimmed = trim.trim_end().len();
    if end_trimmed == 0 {
        return String::new(); // is guaranteed to not allocate. And you'll have to dealloc the empty string at some point...
    }
    let start_trimmed = trim.len() - trim.trim_start().len();

    let mut bytes = trim.into_bytes();
    if start_trimmed != 0 {
        bytes.copy_within(start_trimmed..end_trimmed, 0);
    }

    unsafe {
        bytes.set_len(end_trimmed - start_trimmed);
        String::from_utf8_unchecked(bytes)
    }
}

pub fn drain(mut trim: String) -> String {
    trim.drain(trim.trim_end().len()..);
    trim.drain(..(trim.len() - trim.trim_start().len()));
    trim
}

pub fn r#unsafe(mut trim: String) -> String {
    // rfind would be cleaner, but this is is consise.
    let end_trimmed = trim.trim_end().len();
    if end_trimmed == 0 {
        let mut bytes = trim.into_bytes();
        unsafe {
            bytes.set_len(0);
            return String::from_utf8_unchecked(bytes);
        }
    }
    let start_trimmed = trim.len() - trim.trim_start().len();

    unsafe {
        let bytes = trim.as_mut_vec();
        if start_trimmed != 0 {
            for (from, to) in (start_trimmed..end_trimmed).zip(0..) {
                *bytes.get_unchecked_mut(to) = *bytes.get_unchecked(from)
            }
        }

        bytes.set_len(end_trimmed - start_trimmed);
    }
    trim
}
