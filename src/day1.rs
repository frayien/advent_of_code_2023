use std::fs;

pub fn day1_1(path: &str) -> i32 {
    let content = fs::read_to_string(path).unwrap();

    let mut total = 0;

    for line in content.lines() {

        let mut word: String = "".to_string();

        for c in line.chars() {
            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                word.push(c);
                break;
            }
        }

        total += word.parse::<i32>().unwrap();
    }

    total
}