extern crate libc;
use libc::{c_char,c_void};

#[link(name = "ncurses")]
extern {
    fn initscr() -> c_void;
    fn noecho() -> c_void;
    fn getch() -> c_char;
    fn addch(ch: c_char) -> c_void;
    fn erase() -> c_void;
    fn endwin() -> c_void;
}

fn main() {
    let mut raw_ch: c_char;
    let mut query = "".to_string();
    let mut quit = false;

    unsafe { initscr(); noecho(); }

    loop {
        unsafe { raw_ch = getch(); }
        handle_input(&mut raw_ch, &mut query, &mut quit);

        if quit {
            break;
        } else {
            update_screen(&mut query);
        }
    }

    unsafe { endwin(); }
    println!("{:?}", query);
}

fn handle_input(raw_ch: &mut c_char, query: &mut String, quit: &mut bool) {
    let ch = (*raw_ch as u8) as char;

    match ch {
        // backspace
        '\u{7f}' => {
            let new_len = query.len() - 1;
            query.truncate(new_len);
        },

        // ctrl-u
        '\u{15}' => {
            *query = "".to_string();
        },

        // enter
        '\n' => {
            *quit = true;
        }

        // character
        _ => {
            query.push(ch);
        }
    }
}

fn update_screen(query: &mut String) {
    unsafe {
        erase();
        for ch in query.chars() {
            addch(ch as c_char);
        }
    }
}

//fn fuzzy_match(query: &String, line: &String) -> bool {
//    let mut line_chars = line.chars();
//    let mut result = true;
//
//    for query_char in query.chars() {
//        let mut curr: Option<char>;
//
//        loop {
//            curr = line_chars.next();
//
//            match curr {
//                Some(ch) => {
//                    if query_char.is_uppercase() {
//                        if ch == query_char { break; }
//                    } else {
//                        if ch.to_lowercase() == query_char { break; }
//                    }
//                },
//                None => {
//                    result = false;
//                    break;
//                }
//            }
//        }
//    }
//
//    result
//}
//
//#[test]
//fn test_fuzzy_match() {
//    let query = "ac".to_string();
//    assert!(fuzzy_match(&query, &"abc".to_string()));
//    assert!(fuzzy_match(&query, &"zac".to_string()));
//    assert!(fuzzy_match(&query, &"Abc".to_string()));
//    assert!(fuzzy_match(&query, &"acd".to_string()));
//
//    assert!(!fuzzy_match(&query, &"ab".to_string()));
//    assert!(!fuzzy_match(&query, &"ca".to_string()));
//
//    let cased_query = "aC".to_string();
//    assert!(fuzzy_match(&cased_query, &"abCd".to_string()));
//    assert!(!fuzzy_match(&cased_query, &"abcd".to_string()));
//}
