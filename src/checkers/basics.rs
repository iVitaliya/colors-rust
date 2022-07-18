use crate::checkers::utils;

/// Basics : Replace Close
/// ----------------------
/// This function open & closes the text with coloring. 
pub fn replace_close(index: i32, string: &str, close: &str, replace: &str, head: &str, tail: &str, next: i32) -> String {
    if next < 0 {
        return head.to_string() + &tail.to_string();
    }

    return head.to_string() + &replace_close(next, tail, close, replace, (head.to_string() + &replace.to_string()).as_str(), tail, string.find(close).unwrap() as i32);
}

/// Basics : Clear Bleed
/// --------------------
/// This function clears any bleeding from the coloring.
pub fn clear_bleed(index: i32, string: &str, open: &str, close: &str, replace: &str) -> String {
    if index < 0 {
        return open.to_string() + &string.to_string() + &close.to_string();
    }

    return open.to_string() + &replace_close(index, string, close, replace, "", "", string.find(open).unwrap() as i32);
}

/// Basics : Filter Empty
/// ---------------------
/// This function filters the empty founded strings.
pub fn filter_empty(open: &str, close: &str, replace: &str, at: i32, string: &str) -> String {
    if string == "" || string.len() < 1 {
        return "".to_string();
    }
    
    return clear_bleed(at, string, open, close, replace);
}

/// Initiate
/// ========
/// This function handles the color crafting.
pub fn initiate(open: i32, close: i32, replace: &str, string: &str) -> String {
    let open_val = utils::int_to_string(open);
    let close_val = utils::int_to_string(close);

    return filter_empty(("\x1b[".to_string() + &open_val.to_string() + &"m".to_string()).as_str(), ("\x1b[".to_string() + &close_val+ &"m".to_string()).as_str(), replace, string.find(open_val.as_str()).unwrap() as i32, string)
}