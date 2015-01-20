use std::io::stdin;
use std::os::args;

fn main() {
    let query = args()[1].to_string();
    let mut lines = Vec::new();

    for result in stdin().lock().lines() {
        lines.push(result.unwrap().trim().to_string());
    }

    for line_str in lines.iter() {
        let line = line_str.to_string();

        if fuzzy_match(&query, &line) {
            println!("{}", line);
        }
    }
}

fn fuzzy_match(query: &String, line: &String) -> bool {
    let mut line_chars = line.chars();
    let mut result = true;

    for _query_char in query.chars() {
        let query_char = _query_char.to_lowercase();
        let mut curr: Option<char>;

        loop {
            curr = line_chars.next();

            match curr {
                Some(ch) => {
                    if ch.to_lowercase() == query_char { break; }
                },
                None => {
                    result = false;
                    break;
                }
            }
        }
    }

    result
}

#[test]
fn test_fuzzy_match() {
    let query = "ac".to_string();

    assert!(fuzzy_match(&query, &"abc".to_string()));
    assert!(fuzzy_match(&query, &"zac".to_string()));
    assert!(fuzzy_match(&query, &"Abc".to_string()));
    assert!(fuzzy_match(&query, &"acd".to_string()));

    assert!(!fuzzy_match(&query, &"ab".to_string()));
    assert!(!fuzzy_match(&query, &"ca".to_string()));
}
