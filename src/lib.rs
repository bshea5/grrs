#![allow(unused)]

use std::{io::{BufRead, Lines, Read, BufReader}};

// Using BufReader over std::fs::read_to_string, as its less memory intensive.
// Using Generic, requiring the trait Read for Lines<BufReader<R>> so we can use 
// content from File or content from String.
pub fn find_matches<R: Read>(content: Lines<BufReader<R>>, pattern: &str, mut writer: impl std::io::Write) {
    for line in content {
        let line_str = line.unwrap().to_string();
        if line_str.contains(pattern) {
            writeln!(writer, "{}", line_str)
                .expect("could not write")
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let test_string = "lorem ipsum\ndolor sit amet";
    let reader = BufReader::new(test_string.as_bytes());
    let content = reader.lines();
    find_matches(content, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}