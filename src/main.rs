mod checkers;

use checkers::basics;

fn bg_black(string: &str) -> String {
    return basics::initiate(40, 49, "", string);
}

fn test(string: &str) -> String {
    return bg_black(string);
}

fn main() {
    println!("{}", test("This is a test").as_str())
}