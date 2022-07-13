pub fn ReplaceClose(index: i32, string: &str, close: &str, replace: &str, head: &str, tail: &str, next: i32) -> String {
    if (next < 0) {
        return head + tail;
    }

    return head + ReplaceClose(next, tail, close, replace, head+replace, tail, string.find(close).unwrap() as i32);
}

pub fn ClearBleed(index: i32, string: &str, open: &str, close: &str, replace: &str) -> String {
    if (index < 0) {
        return open + string + close;
    }

    return open + ReplaceClose(index, string, close, replace, "", "", string.find(open).unwrap() as i32);
}

pub fn FilterEmpty(open: &str, close: &str, replace: &str, at: i32) {
    return fn (string: &str) -> String {
        if (string == "" || string.len()) {
            return "";
        }

        return ClearBleed(at, str, open, close, replace);
    }
}

