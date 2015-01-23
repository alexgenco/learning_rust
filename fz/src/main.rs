use std::io::stdin;
use std::os::args;
use std::io::File;
use std::io::Command;

fn main() {
    let mut lines = Vec::new();

    for result in stdin().lock().lines() {
        lines.push(result.unwrap().trim().to_string());
    }

    read_loop(lines);
}

fn read_loop(lines: Vec<String>) {
    Command::new("/bin/stty").arg("-echo").arg("-icanon").arg("-icrnl").spawn().unwrap().wait();

    let tty_path = Path::new("/dev/tty");
    let mut tty = File::open(&tty_path);
    let mut query = "".to_string();
    let mut input: char;

    loop {
        input = tty.read_byte().unwrap() as char;
        let idx = query.len();
        query.insert(idx, input);

        for line_str in lines.iter() {
            let line = line_str.to_string();

            if fuzzy_match(&query, &line) {
                println!("{}", line);
            }
        }
    }

    Command::new("/bin/stty").arg("sane").spawn().unwrap().wait();
}

fn fuzzy_match(query: &String, line: &String) -> bool {
    let mut line_chars = line.chars();
    let mut result = true;

    for query_char in query.chars() {
        let mut curr: Option<char>;

        loop {
            curr = line_chars.next();

            match curr {
                Some(ch) => {
                    if query_char.is_uppercase() {
                        if ch == query_char { break; }
                    } else {
                        if ch.to_lowercase() == query_char { break; }
                    }
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

    let cased_query = "aC".to_string();
    assert!(fuzzy_match(&cased_query, &"abCd".to_string()));
    assert!(!fuzzy_match(&cased_query, &"abcd".to_string()));
}
