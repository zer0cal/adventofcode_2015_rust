// --- Day 8: Matchsticks ---

#![allow(dead_code)]
use std::fs;

pub fn answer() {
    println!("Day 8: Matchsticks");
    let input = fs::read_to_string("day8_input.txt").expect("err reading day 8 input");
    let (characters, codes) = count_characters_and_utf_codes(&input);
    println!(
        "answer to pt 1 is {} - {} = {}",
        codes,
        characters,
        codes - characters
    );

    let encoded = encode_str_into_utf_codes(&input);
    let (characters_encoded, codes_encoded) = count_characters_and_utf_codes(&encoded);
    println!(
        "answer to pt 2 is (codes) {} - {} = {}",
        codes_encoded,
        codes,
        codes_encoded - codes
    );
    println!(
        "answer to pt 2 is (characters) {} - {} = {}",
        characters_encoded,
        characters,
        characters_encoded - characters
    );
}

fn count_characters_and_utf_codes(s: &str) -> (u32, u32) {
    let mut characters = 0;
    let mut codes = 0;
    let mut c = s.bytes().into_iter();
    loop {
        match c.next() {
            Some(13) => (),
            Some(10) => (),
            Some(b'\"') => {
                codes += 1;
            }
            Some(b'\\') => {
                codes += 1;
                match c.next() {
                    Some(b'x') => {
                        codes += 3;
                        characters += 1;
                        c.next();
                        c.next();
                    }
                    None => {
                        break;
                    }
                    Some(_) => {
                        codes += 1;
                        characters += 1;
                    }
                }
            }
            None => {
                break;
            }
            Some(_) => {
                codes += 1;
                characters += 1;
            }
        }
    }
    (characters, codes as u32)
}

fn encode_to_utf_code(input: u8) -> String {
    return match input {
        b'\"' => String::from(r#"\""#),
        b'\\' => String::from(r#"\\"#),
        // 13 => String::from(r#"\r"#),
        13 => String::from(r#""""#),
        // 10 => String::from(r#"\n"#),
        10 => String::from(r#""#),
        _ => (input as char).to_string(), // _ => &"_", // u => &String::from(char::from(*u)), // u => String::from_utf8(vec![*u]).unwrap(),
    };
}

fn encode_str_into_utf_codes(s: &str) -> String {
    let lines = s.split_whitespace();
    let mut res = String::new();
    for line in lines {
        res.push('\"');
        line.bytes()
            .for_each(|u| res.push_str(&encode_to_utf_code(u)));
        res.push('\"');
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::day8::{count_characters_and_utf_codes, encode_str_into_utf_codes};
    use std::fs;

    #[test]
    fn first_line_from_file() {
        let input =
            fs::read_to_string(r"src\day8_input_first_line.txt").expect("err reading day 8 input");
        let (characters, codes) = count_characters_and_utf_codes(&input);
        assert_eq!((characters, codes), (7, 9));
    }

    #[test]
    fn oneline() {
        let s = r#""aaa""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (3, 5));
    }

    #[test]
    fn empty() {
        let s = r#"""
""
"""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (0, 6));
    }

    #[test]
    fn singlebackslash() {
        let s = r#""\\""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (1, 4));
    }

    #[test]
    fn doublequote() {
        let s = r#""\"""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (1, 4));
    }

    #[test]
    fn ascii_character() {
        let s = r#""\x11""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (1, 6));
    }

    #[test]
    fn multiline() {
        let s = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let (characters, codes) = count_characters_and_utf_codes(s);
        assert_eq!((characters, codes), (11, 23));
    }

    #[test]
    fn encoder_empty() {
        let c = r#""""#;
        let res: String = encode_str_into_utf_codes(c);
        assert_eq!(res, r#""\"\"""#)
    }

    #[test]
    fn encoder_abc() {
        let c = r#""abc""#;
        let res: String = encode_str_into_utf_codes(c);
        assert_eq!(res, r#""\"abc\"""#)
    }

    #[test]
    fn encoder_quote() {
        let c = r#""aaa\"aaa""#;
        let res: String = encode_str_into_utf_codes(c);
        assert_eq!(res, r#""\"aaa\\\"aaa\"""#)
    }

    #[test]
    fn encoder_utf_code() {
        let c = r#""\x27""#;
        let res: String = encode_str_into_utf_codes(c);
        assert_eq!(res, r#""\"\\x27\"""#)
    }

    #[test]
    fn encoder_first_line_from_file() {
        let input =
            fs::read_to_string(r"src\day8_input_first_line.txt").expect("err reading day 8 input");
        println!("{}", input);
        println!("{:?}", input.bytes());
        let res: String = encode_str_into_utf_codes(&input);
        assert_eq!(res, r#""\"qxfcsmh\"""#)
    }
}
