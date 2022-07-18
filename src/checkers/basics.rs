use checkers::utils;

pub fn replace_close(index: i32, string: &str, close: &str, replace: &str, head: &str, tail: &str, next: i32) -> String {
    if next < 0 {
        return head + tail;
    }

    return head + replace_close(next, tail, close, replace, head+replace, tail, string.find(close).unwrap() as i32);
}

pub fn clear_bleed(index: i32, string: &str, open: &str, close: &str, replace: &str) -> String {
    if index < 0 {
        return open + string + close;
    }

    return open + replace_close(index, string, close, replace, "", "", string.find(open).unwrap() as i32);
}

pub fn filter_empty(open: &str, close: &str, replace: &str, at: i32) -> impl Fn(&str) -> String {
    fn f(string: &str) -> String {
        if string == "" || string.len() < 1 {
            return "".to_string();
        }

        return clear_bleed(at, str, open, close, replace);
    }

    return f;
}

/// Initiate
/// ========
/// This function handles the color crafting.
pub fn initiate(open: i32, close: i32, replace: &str) -> impl Fn(&str) -> &str {
    let open_val = utils::int_to_string(open);
    let close_val = utils::int_to_string(close);

    fn i(s: &str) -> String {
        return filter_empty("\x1b["+open_val+"m", "\x1b["+close_val+"m", replace, s.find(open_val).unwrap() as i32)
    }

    return i;
}