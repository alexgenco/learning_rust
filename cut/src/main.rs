#[cfg(not(test))]
use std::io;

#[cfg(not(test))]
fn main() {
    for line in io::stdin().lines() {
        println!("{}", line.unwrap());
    }
}

fn cut(line: String, delim: char) -> Vec<String> {
    vec!["test".to_string()]
}

#[test]
fn it_splits_on_a_delimiter() {
    let line: String = "one-two three".to_string();
    let result: Vec<String> = cut(line, '-');
    assert_eq!(vec!["one".to_string(), "two three".to_string()], result);
}
