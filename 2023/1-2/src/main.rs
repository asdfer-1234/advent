use std::fs;

const DIGITS: [(&str, u8); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn digit_forward(s: &str) -> Option<u8> {
    for (pattern, value) in DIGITS {
        if s.starts_with(pattern) {
            return Some(value);
        }
    }
    None
}

fn search_digit_forward(s: &str) -> u8 {
    let len = s.len();
    for i in 0..len {
        let digit = digit_forward(&s[i..len]);
        if let Some(i) = digit {
            return i;
        }
    }
    panic!()
}

fn digit_backward(s: &str) -> Option<u8> {
    for (pattern, value) in DIGITS {
        if s.ends_with(pattern) {
            return Some(value);
        }
    }
    None
}

fn search_digit_backward(s: &str) -> u8 {
    let len = s.len();
    for i in (1..=len).rev() {
        let digit = digit_backward(&s[0..i]);
        if let Some(i) = digit {
            return i;
        }
    }
    panic!()
}

fn integer_from_line_slice(s: &str) -> u8 {
    let first_digit: u8 = search_digit_forward(s);
    let last_digit: u8 = search_digit_backward(s);

    first_digit * 10 + last_digit
}

fn main() {
    let input_string = fs::read_to_string("input").unwrap();

    let mut slice = String::new();
    let mut sum: u32 = 0;

    for c in input_string.chars() {
        match c {
            '\n' => {
                sum += integer_from_line_slice(&slice) as u32;
                slice = String::new();
            }
            _ => {
                slice.push(c);
            }
        }
    }

    println!("{}", sum);
}
