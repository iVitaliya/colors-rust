pub fn ReplaceClose(index: i32, string: &str, close: &str, replace: &str, head: &str, tail: &str, next: i32) -> String {
    if (next < 0) {
        return head + tail;
    }

    return head + ReplaceClose()
}